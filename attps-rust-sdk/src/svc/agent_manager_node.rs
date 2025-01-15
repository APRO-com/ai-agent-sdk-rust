use std::sync::Arc;
use tokio::time::{sleep, Duration};
use regex::Regex;
use hex;

use ethers::prelude::*;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::types::{Address, H256};

use crate::core::ApiResult;

abigen!(
    AgentManagerContract,
    "./src/abi/AgentManager.json"
);

pub struct AgentManagerNode {
    contract: AgentManagerContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl AgentManagerNode {
    pub async fn new(provider_url: &str, contract_addr: &str, private_key: &str) -> Self {
        let provider = Provider::<Http>::try_from(provider_url).expect("Invalid provider URL");
        let chain_id = provider.get_chainid().await.expect("Failed to get chain ID").as_u64();
        let wallet: Wallet<SigningKey> = private_key.parse().expect("Invalid private key");
        let wallet = wallet.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let provider = Arc::new(client);

        let address = contract_addr.parse::<Address>().unwrap();
        let contract = AgentManagerContract::new(address, provider.clone());
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

    pub async fn agent_proxy(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.agent_proxy().call().await
                .map(|address| format!("{:?}", address))
                .map_err(|_| "Failed to get agent proxy".to_string())
        }).await
    }

    pub async fn get_owner(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.owner().call().await
                .map(|address| format!("{:?}", address))
                .map_err(|_| "Failed to get owner".to_string())
        }).await
    }

    pub async fn get_type_and_version(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.type_and_version().call().await
                .map_err(|_| "Failed to get type and version".to_string())
        }).await
    }

    pub async fn agent_version(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.agent_version().call().await
                .map_err(|_| "Failed to get agent version".to_string())
        }).await
    }

    pub async fn allowed_agent(&self, agent_address: &str) -> ApiResult<bool> {
        let address = agent_address.parse::<Address>().map_err(|_| {
            "Invalid address format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.allowed_agent(address).call().await
                .map_err(|_| "Failed to check if agent is allowed".to_string())
        }).await
    }

    pub async fn allowed_signer(&self, agent_address: &str, setting_digest: &str, signer_address: &str) -> ApiResult<bool> {
        let agent = agent_address.parse::<Address>().map_err(|_| {
            "Invalid agent address format".to_string()
        })?;

        let setting_digest = setting_digest.parse::<H256>().map_err(|_| {
            "Invalid setting digest format".to_string()
        })?;

        let signer = signer_address.parse::<Address>().map_err(|_| {
            "Invalid signer address format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.allowed_signer(agent, setting_digest.into(), signer).call().await
                .map_err(|_| "Failed to check if signer is allowed".to_string())
        }).await
    }

    pub async fn get_agent_config(&self, agent_address: &str, setting_digest: &str) -> ApiResult<String> {
        let agent = agent_address.parse::<Address>().map_err(|_| {
            "Invalid agent address format".to_string()
        })?;

        let setting_digest = setting_digest.parse::<H256>().map_err(|_| {
            "Invalid setting digest format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.get_agent_config(agent, setting_digest.into()).call().await
                .map(|config| format!("{:?}", config))
                .map_err(|_| "Failed to get agent config".to_string())
        }).await
    }

    pub async fn get_agent_configs(&self, agent_address: &str) -> ApiResult<String> {
        let agent = agent_address.parse::<Address>().map_err(|_| {
            "Invalid agent address format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.get_agent_configs(agent).call().await
                .map(|configs| format!("{:?}", configs))
                .map_err(|_| "Failed to get agent configs".to_string())
        }).await
    }

    pub async fn get_agent_configs_count(&self, agent_address: &str) -> ApiResult<u64> {
        let agent = agent_address.parse::<Address>().map_err(|_| {
            "Invalid agent address format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.get_agent_configs_count(agent).call().await
                .map_err(|_| "Failed to get agent configs count".to_string())
        }).await
    }

    pub async fn get_agent_configs_in_range(&self, agent_address: &str, agent_config_idx_start: u64, agent_config_idx_end: u64) -> ApiResult<String> {
        let agent = agent_address.parse::<Address>().map_err(|_| {
            "Invalid agent address format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.get_agent_configs_in_range(agent, agent_config_idx_start, agent_config_idx_end).call().await
                .map(|configs| format!("{:?}", configs))
                .map_err(|_| "Failed to get agent configs in range".to_string())
        }).await
    }

    pub async fn get_all_allowed_agents(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_all_allowed_agents().call().await
                .map(|agents| format!("{:?}", agents))
                .map_err(|_| "Failed to get all allowed agents".to_string())
        }).await
    }

    pub async fn get_all_registering_agents(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_all_registering_agents().call().await
                .map(|agents| format!("{:?}", agents))
                .map_err(|_| "Failed to get all registering agents".to_string())
        }).await
    }

    pub async fn get_allowed_agents_count(&self) -> ApiResult<u64> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_allowed_agents_count().call().await
                .map_err(|_| "Failed to get allowed agents count".to_string())
        }).await
    }

    pub async fn get_allowed_agents_in_range(&self, allowed_agent_idx_start: u64, allowed_agent_idx_end: u64) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_allowed_agents_in_range(allowed_agent_idx_start, allowed_agent_idx_end).call().await
                .map(|agents| format!("{:?}", agents))
                .map_err(|_| "Failed to get allowed agents in range".to_string())
        }).await
    }

    pub async fn get_registering_agents_count(&self) -> ApiResult<u64> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_registering_agents_count().call().await
                .map_err(|_| "Failed to get registering agents count".to_string())
        }).await
    }

    pub async fn get_registering_agents_in_range(&self, registering_agent_idx_start: u64, registering_agent_idx_end: u64) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.get_registering_agents_in_range(registering_agent_idx_start, registering_agent_idx_end).call().await
                .map(|agents| format!("{:?}", agents))
                .map_err(|_| "Failed to get registering agents in range".to_string())
        }).await
    }

    pub async fn is_valid_message_id(&self, message_id: &str) -> ApiResult<bool> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.is_valid_message_id(message_id.to_string()).call().await
                .map_err(|_| "Failed to validate message ID".to_string())
        }).await
    }

    pub async fn is_valid_source_agent_id(&self, source_agent_id: &str) -> ApiResult<bool> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.is_valid_source_agent_id(source_agent_id.to_string()).call().await
                .map_err(|_| "Failed to validate source agent ID".to_string())
        }).await
    }

    pub async fn signer_threshold(&self, agent_address: &str, setting_digest: &str) -> ApiResult<u8> {
        let agent = agent_address.parse::<Address>().map_err(|_| {
            "Invalid agent address format".to_string()
        })?;

        let setting_digest = setting_digest.parse::<H256>().map_err(|_| {
            "Invalid setting digest format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.signer_threshold(agent, setting_digest.into()).call().await
                .map_err(|_| "Failed to get signer threshold".to_string())
        }).await
    }

    pub async fn validate_data_conversion(&self, agent_address: &str, data: &str) -> ApiResult<String> {
        let agent = agent_address.parse::<Address>().map_err(|_| {
            "Invalid agent address format".to_string()
        })?;

        let data_bytes = hex::decode(data.trim_start_matches("0x")).map_err(|_| {
            "Invalid data format".to_string()
        })?;

        self.retry_with_exponential_backoff(|| async {
            self.contract.validate_data_conversion(agent, data_bytes.clone().into()).call().await
                .map(|converted_data| format!("{:?}", converted_data))
                .map_err(|_| "Failed to validate data conversion".to_string())
        }).await
    }

    pub async fn accept_agent(&self, agent_address: &str) -> ApiResult<String> {
        let agent_address = agent_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format: {}", e)
        })?;

        let contract_call = self.contract.accept_agent(agent_address);

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }

    pub async fn accept_agent_setting_proposal(&self, agent_address: &str) -> ApiResult<String> {
        let agent_address = agent_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format: {}", e)
        })?;

        let contract_call = self.contract.accept_agent_setting_proposal(agent_address);

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }

    pub async fn accept_ownership(&self) -> ApiResult<String> {
        let contract_call = self.contract.accept_ownership();

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }

    pub async fn change_agent_setting_proposal(
        &self,
        agent_address: &str,
        signers: Vec<&str>,
        threshold: u8,
        converter_address: &str,
        version: &str,
        message_id: &str,
        source_agent_id: &str,
        source_agent_name: &str,
        target_agent_id: &str,
        timestamp: u64,
        message_type: u8,
        priority: u8,
        ttl: u64,
    ) -> ApiResult<String> {
        let agent_address = agent_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format for agent: {}", e)
        })?;

        let signers: Vec<Address> = signers
            .iter()
            .map(|s| s.parse::<Address>())
            .collect::<Result<_, _>>()
            .map_err(|e| format!("Invalid address format in signers: {}", e))?;

        let converter_address = converter_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format for converter address: {}", e)
        })?;

        let agent_header = AgentHeader {
            version: version.to_string(),
            message_id: message_id.to_string(),
            source_agent_id: source_agent_id.to_string(),
            source_agent_name: source_agent_name.to_string(),
            target_agent_id: target_agent_id.to_string(),
            timestamp: U256::from(timestamp),
            message_type,
            priority,
            ttl: U256::from(ttl),
        };

        let agent_settings = AgentSettings {
            signers,
            threshold,
            converter_address,
            agent_header,
        };

        let contract_call = self.contract.change_agent_setting_proposal(agent_address, agent_settings);

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }

    pub async fn register_agent(
        &self,
        agent_address: &str,
        signers: Vec<&str>,
        threshold: u8,
        converter_address: &str,
        version: &str,
        message_id: &str,
        source_agent_id: &str,
        source_agent_name: &str,
        target_agent_id: &str,
        timestamp: u64,
        message_type: u8,
        priority: u8,
        ttl: u64,
    ) -> ApiResult<String> {
        let agent_address = agent_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format for agent: {}", e)
        })?;

        let signers: Vec<Address> = signers
            .iter()
            .map(|s| s.parse::<Address>())
            .collect::<Result<_, _>>()
            .map_err(|e| format!("Invalid address format in signers: {}", e))?;

        let converter_address = converter_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format for converter address: {}", e)
        })?;

        let agent_header = AgentHeader {
            version: version.to_string(),
            message_id: message_id.to_string(),
            source_agent_id: source_agent_id.to_string(),
            source_agent_name: source_agent_name.to_string(),
            target_agent_id: target_agent_id.to_string(),
            timestamp: U256::from(timestamp),
            message_type,
            priority,
            ttl: U256::from(ttl),
        };

        let agent_settings = AgentSettings {
            signers,
            threshold,
            converter_address,
            agent_header,
        };

        let contract_call = self.contract.register_agent(agent_address, agent_settings);

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }

    pub async fn remove_agent(&self, agent_address: &str) -> ApiResult<String> {
        let agent_address = agent_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format for agent: {}", e)
        })?;

        let contract_call = self.contract.remove_agent(agent_address);

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }

    pub async fn set_agent_proxy(&self, proxy_address: &str) -> ApiResult<String> {
        let proxy_address = proxy_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format for proxy: {}", e)
        })?;

        let contract_call = self.contract.set_agent_proxy(proxy_address);

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }

    pub async fn transfer_ownership(&self, new_owner_address: &str) -> ApiResult<String> {
        let new_owner_address = new_owner_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format for new owner: {}", e)
        })?;

        let contract_call = self.contract.transfer_ownership(new_owner_address);

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }
}

pub fn extract_setting_digests(configs: &str) -> Vec<H256> {
    let mut setting_digests = Vec::new();
    let re = Regex::new(r"config_digest: \[([^\]]+)\]").unwrap();

    for cap in re.captures_iter(configs) {
        if let Some(digest_str) = cap.get(1) {
            let digest_bytes: Vec<u8> = digest_str.as_str()
                .split(", ")
                .filter_map(|s| s.parse::<u8>().ok())
                .collect();

            if digest_bytes.len() == 32 {
                let mut digest_array = [0u8; 32];
                digest_array.copy_from_slice(&digest_bytes);
                setting_digests.push(H256::from(digest_array));
            }
        }
    }

    setting_digests
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::H256;

    #[test]
    fn test_extract_setting_digests() {
        let input = r#"
            AgentConfig { config_digest: [1, 0, 229, 66, 143, 97, 153, 92, 162, 246, 29, 150, 178, 77, 144, 180, 141, 229, 139, 129, 140, 201, 29, 187, 136, 193, 191, 116, 232, 61, 243, 203], config_block_number: 7476906, is_active: true, settings: AgentSettings { signers: [0x9538e13c0e111c5b0525f1592079aa1586b4e9cc, 0x83390ef6b20a29ccbf0955567556af519e86a958], threshold: 2, converter_address: 0x0000000000000000000000000000000000000000, agent_header: AgentHeader { version: "1.0", message_id: "4b0aa564-0871-42da-bc6a-6c09a5d0173a", source_agent_id: "4b0aa564-0871-42da-bc6a-6c09a5d0173a", source_agent_name: "SourceAgent", target_agent_id: "4b0aa564-0871-42da-bc6a-6c09a5d0173a", timestamp: 1700000000, message_type: 0, priority: 1, ttl: 3600 } } }
            AgentConfig { config_digest: [1, 0, 229, 66, 143, 97, 153, 92, 162, 246, 29, 150, 178, 77, 144, 180, 141, 229, 139, 129, 140, 201, 29, 187, 136, 193, 191, 116, 232, 61, 243, 203], config_block_number: 7476906, is_active: true, settings: AgentSettings { signers: [0x9538e13c0e111c5b0525f1592079aa1586b4e9cc, 0x83390ef6b20a29ccbf0955567556af519e86a958], threshold: 2, converter_address: 0x0000000000000000000000000000000000000000, agent_header: AgentHeader { version: "1.0", message_id: "4b0aa564-0871-42da-bc6a-6c09a5d0173a", source_agent_id: "4b0aa564-0871-42da-bc6a-6c09a5d0173a", source_agent_name: "SourceAgent", target_agent_id: "4b0aa564-0871-42da-bc6a-6c09a5d0173a", timestamp: 1700000000, message_type: 0, priority: 1, ttl: 3600 } } }
        "#;

        let expected_digest = H256::from([
            1, 0, 229, 66, 143, 97, 153, 92, 162, 246, 29, 150, 178, 77, 144, 180, 141, 229, 139, 129, 140, 201, 29, 187, 136, 193, 191, 116, 232, 61, 243, 203
        ]);

        let digests = extract_setting_digests(input);
        assert_eq!(digests.len(), 2);
        assert_eq!(digests[0], expected_digest);
        assert_eq!(digests[1], expected_digest);
    }
}