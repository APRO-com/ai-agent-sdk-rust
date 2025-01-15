use once_cell::sync::Lazy;

pub static AGENTPROXY_CONTRACT: Lazy<String> =
    Lazy::new(|| std::env::var("AGENTPROXY_CONTRACT").expect("env not found AGENTPROXY_CONTRACT"));

pub static AGENTPROXY_PROVIDER: Lazy<String> =
    Lazy::new(|| std::env::var("AGENTPROXY_PROVIDER").expect("env not found AGENTPROXY_PROVIDER"));

pub static WALLET_PRIVATE_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("WALLET_PRIVATE_KEY").expect("env not found WALLET_PRIVATE_KEY"));
