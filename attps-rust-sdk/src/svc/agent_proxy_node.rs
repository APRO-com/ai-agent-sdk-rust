use std::sync::Arc;
use tokio::time::{sleep, Duration};

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::signers::Wallet;
use ethers::types::{Address, H256};
use hex;

use crate::core::ApiResult;

abigen!(
    AgentProxyContract,
    "./src/abi/AgentProxy.json"
);

pub struct AgentProxyNode {
    contract: AgentProxyContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl AgentProxyNode {
    pub async fn new(provider_url: &str, contract_addr: &str, private_key: &str) -> Self {
        let provider = Provider::<Http>::try_from(provider_url).expect("Invalid provider URL");
        let chain_id = provider.get_chainid().await.expect("Failed to get chain ID").as_u64();
        let wallet: Wallet<SigningKey> = private_key.parse().expect("Invalid private key");
        let wallet = wallet.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let provider = Arc::new(client);

        let address = contract_addr.parse::<Address>().unwrap();
        let contract = AgentProxyContract::new(address, provider.clone());
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

    pub async fn get_agent_factory(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.agent_factory().call().await
                .map(|address| format!("{:?}", address))
                .map_err(|_| "Failed to get agent factory".to_string())
        }).await
    }

    pub async fn get_agent_manager(&self) -> ApiResult<String> {
        self.retry_with_exponential_backoff(|| async {
            self.contract.agent_manager().call().await
                .map(|address| format!("{:?}", address))
                .map_err(|_| "Failed to get agent manager".to_string())
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

    pub async fn accept_ownership(&self) -> ApiResult<String> {
        let contract_call = self.contract.accept_ownership();

        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;

        let contract_call_with_gas = contract_call.gas(gas_estimate);

        let pending_tx = contract_call_with_gas
            .send()
            .await
            .map_err(|e| format!("Failed to send accept_ownership transaction: {:?}", e))?;

        let receipt = pending_tx.await.map_err(|e| {
            format!("Transaction failed: {:?}", e)
        })?;

        if let Some(receipt) = receipt {
            Ok(format!("{:?}", receipt))
        } else {
            Err("Transaction did not return a receipt".to_string())
        }
    }
    

    pub async fn set_agent_factory(&self, factory_address: &str) -> ApiResult<String> {
        let factory_address = factory_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format: {}", e)
        })?;
    
        let contract_call = self.contract.set_agent_factory(factory_address);
    
        let gas_estimate = contract_call.estimate_gas().await.map_err(|e| {
            format!("Failed to estimate gas: {:?}", e)
        })?;
        println!("Gas estimate: {}", gas_estimate);
    
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
    
    pub async fn set_agent_manager(&self, manager_address: &str) -> ApiResult<String> {
        let manager_address = manager_address.parse::<Address>().map_err(|e| {
            format!("Invalid address format: {}", e)
        })?;

        let contract_call = self.contract.set_agent_manager(manager_address);

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
            format!("Invalid address format: {}", e)
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

    pub async fn create_and_register_agent(
        &self,
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

        let contract_call = self.contract.create_and_register_agent(agent_settings);

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

    pub async fn verify(
        &self,
        agent: &str,
        settings_digest: &str,
        data: &str,
        data_hash: &str,
        signature_proof: &str,
        zk_proof: &str,
        merkle_proof: &str,
        content_type: &str,
        encoding: &str,
        compression: &str,
    ) -> ApiResult<String> {
        let agent = agent.parse::<Address>().map_err(|e| {
            format!("Invalid agent address format: {}", e)
        })?;
        let settings_digest: [u8; 32] = settings_digest.parse::<H256>().map_err(|e| {
            format!("Invalid settings digest format: {}", e)
        })?.into();
        let data = hex::decode(data).map_err(|e| {
            format!("Invalid data format: {}", e)
        })?;
        let data_hash: [u8; 32] = data_hash.parse::<H256>().map_err(|e| {
            format!("Invalid data hash format: {}", e)
        })?.into();

        let signature_proof = if signature_proof.starts_with("0x") {
            &signature_proof[2..]
        } else {
            signature_proof
        };
        let signature_proof = hex::decode(signature_proof).map_err(|e| {
            format!("Invalid signature proof format: {}", e)
        })?;

        let zk_proof = if zk_proof.starts_with("0x") {
            &zk_proof[2..]
        } else {
            zk_proof
        };
        let zk_proof = hex::decode(zk_proof).map_err(|e| {
            format!("Invalid zk proof format: {}", e)
        })?;

        let merkle_proof = if merkle_proof.starts_with("0x") {
            &merkle_proof[2..]
        } else {
            merkle_proof
        };
        let merkle_proof = hex::decode(merkle_proof).map_err(|e| {
            format!("Invalid merkle proof format: {}", e)
        })?;

        let proofs = Proofs {
            signature_proof: signature_proof.into(),
            zk_proof: zk_proof.into(),
            merkle_proof: merkle_proof.into(),
        };

        let metadata = Metadata {
            content_type: content_type.to_string(),
            encoding: encoding.to_string(),
            compression: compression.to_string(),
        };

        let message_payload = MessagePayload {
            data: data.into(),
            data_hash,
            proofs,
            metadata,
        };

        let contract_call = self.contract.verify(
            agent,
            settings_digest,
            message_payload,
        );

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
