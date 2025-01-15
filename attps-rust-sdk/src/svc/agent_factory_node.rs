use std::sync::Arc;

use tokio::time::{sleep, Duration};

use ethers::prelude::*;
use ethers::signers::Wallet;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::types::Address;

use crate::core::ApiResult;

abigen!(
    AgentFactoryContract,
    "./src/abi/AgentFactory.json"
);

pub struct AgentFactoryNode {
    contract: AgentFactoryContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl AgentFactoryNode {
    pub async fn new(provider_url: &str, contract_addr: &str, private_key: &str) -> Self {
        let provider = Provider::<Http>::try_from(provider_url).expect("Invalid provider URL");
        let chain_id = provider.get_chainid().await.expect("Failed to get chain ID").as_u64();
        let wallet: Wallet<SigningKey> = private_key.parse().expect("Invalid private key");
        let wallet = wallet.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let provider = Arc::new(client);

        let address = contract_addr.parse::<Address>().unwrap();
        let contract = AgentFactoryContract::new(address, provider.clone());
        Self { contract }
    }

    async fn retry_with_exponential_backoff<F, Fut, T>(&self, mut f: F) -> ApiResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ApiResult<T>>,
    {
        let mut attempts = 0;
        let max_attempts = 3;
        let mut delay = Duration::from_millis(100);

        while attempts < max_attempts {
            match f().await {
                Ok(result) => return Ok(result),
                Err(_) if attempts < max_attempts - 1 => {
                    sleep(delay).await;
                    delay *= 2;
                }
                Err(e) => return Err(e),
            }
            attempts += 1;
        }

        Err("Max retry attempts reached".to_string())
    }

    pub async fn get_agent_manager(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.agent_manager().call().await
                .map(|address| format!("{:?}", address))
                .map_err(|_| "Failed to get agent manager".to_string())
        }).await
    }

    pub async fn get_agent_proxy(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.agent_proxy().call().await
                .map(|address| format!("{:?}", address))
                .map_err(|_| "Failed to get agent proxy".to_string())
        }).await
    }

    pub async fn get_agents_count(&self) -> ApiResult<u64> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_agents_count().call().await
                .map_err(|_| "Failed to get agents count".to_string())
        }).await
    }

    pub async fn get_agents_in_range(&self, agent_idx_start: u64, agent_idx_end: u64) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_agents_in_range(agent_idx_start, agent_idx_end).call().await
                .map(|agents| format!("{:?}", agents))
                .map_err(|_| "Failed to get agents in range".to_string())
        }).await
    }

    pub async fn get_all_agents(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_all_agents().call().await
                .map(|agents| format!("{:?}", agents))
                .map_err(|_| "Failed to get all agents".to_string())
        }).await
    }

    pub async fn has_agent(&self, agent: &str) -> ApiResult<bool> {
        let agent_address = agent.parse::<Address>().map_err(|_| {
            "Invalid address format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.has_agent(agent_address).call().await
                .map_err(|_| "Failed to check if agent exists".to_string())
        }).await
    }

    pub async fn type_and_version(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.type_and_version().call().await
                .map_err(|_| "Failed to get type and version".to_string())
        }).await
    }
}
