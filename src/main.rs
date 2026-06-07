use std::borrow::Cow;
use std::convert::Infallible;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use ahash::AHashMap;
use base64_simd::STANDARD;
use color_eyre::eyre::Result;
use nyquest::header::USER_AGENT;
use nyquest::{ClientBuilder, Request};
use url::{Host, Url};

use ip_replacement::model::{Config, Subscription};

const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION"));

fn open_file(path: impl AsRef<Path>) -> std::io::Result<File> {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path.as_ref())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    nyquest_preset::register();

    let mut config = String::new();
    OpenOptions::new()
        .read(true)
        .write(false)
        .open("./config.toml")?
        .read_to_string(&mut config)?;
    let config: Config = soml::from_str(&config)?;

    let client = ClientBuilder::default().build_blocking()?;

    let sub_dir = PathBuf::from("subs");

    for Subscription {
        url,
        user_agent,

        name,
        ip,
    } in config.subscription
    {
        let js_path = sub_dir.join(&name).with_extension("flclash.js");
        let sub_path = sub_dir.join(&name).with_extension("sub");

        let mut js_file = BufWriter::new(open_file(&js_path)?);
        let mut sub_file = BufWriter::new(open_file(&sub_path)?);

        let request = Request::<Infallible>::get(url).with_header(
            USER_AGENT,
            user_agent
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed(DEFAULT_USER_AGENT)),
        );

        let sip002_sub = client.request(request)?.with_successful_status()?.bytes()?;
        let decoded = STANDARD.decode_to_vec(&sip002_sub)?;

        // left HashMap because we may add regex -> IP mapping
        let mut hosts_map = AHashMap::new();

        let mut count = 0;
        for line in String::from_utf8_lossy(&decoded)
            .lines()
            .map(str::trim)
            .filter(|&s| !s.is_empty())
        {
            let mut url = Url::parse(line)?;
            let host = url.host().as_ref().map(Host::to_string).unwrap_or_default();
            if !matches!(&*host, "www.google.com" | "www.g00gle.com") {
                hosts_map.entry(host).insert_entry(ip.clone());
            }
            url.set_host(Some(&ip))?;
            writeln!(sub_file, "{url}")?;
            count += 1;
        }

        eprintln!(
            "{}: fetched {count} subscriptions",
            sub_path.to_string_lossy(),
        );

        writeln!(
            js_file,
            "const main = (config) => {{
  config.hosts ??= {{}};"
        )?;

        for (host, ip) in &hosts_map {
            writeln!(js_file, "  config.hosts['{host}'] = \"{ip}\";")?;
        }

        writeln!(
            js_file,
            "  return config;
}};"
        )?;

        eprintln!(
            "{}: written {} host pairs",
            js_path.to_string_lossy(),
            hosts_map.len()
        );
    }

    Ok(())
}
