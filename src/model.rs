use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub subscription: Vec<Subscription>,
}

#[derive(Deserialize)]
pub struct Subscription {
    /// Subscription name
    pub name: String,
    /// Subscription URL, can be HTTP/HTTPS
    pub url: String,
    /// `xxx.xxx.xxx.xxx` for ipv4
    ///
    /// `[xx:xx:...]` for ipv6
    pub ip: String,
    /// Optional user-agent, for providers distributes configuration based on that
    pub user_agent: Option<String>,
}
