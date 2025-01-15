mod svc;
pub use svc::agent_factory_node::AgentFactoryNode;
pub use svc::agent_manager_node::{AgentManagerNode, extract_setting_digests};
pub use svc::agent_proxy_node::{AgentProxyNode, Proofs, Metadata};

mod core;
pub use core::ApiResult;
pub use core::consts::{AGENTPROXY_CONTRACT, AGENTPROXY_PROVIDER, WALLET_PRIVATE_KEY};


mod utils;
pub use utils::common::generate_signature_proof;

