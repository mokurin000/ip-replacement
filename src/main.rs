use std::fs::OpenOptions;
use std::io::Read;

use color_eyre::eyre::Result;
use ip_replacement::model::Config;
use nyquest::ClientBuilder;

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

    let client = ClientBuilder::default()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            '/',
            env!("CARGO_PKG_VERSION")
        ))
        .build_blocking()?;

    for sub in config.subscription {}

    Ok(())
}
