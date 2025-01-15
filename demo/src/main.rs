use std::env;

use dotenvy::dotenv;
use ethers::types::Bytes;
use hex;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;


use attps_rust_sdk::{
    AgentFactoryNode,
    AgentManagerNode,
    AgentProxyNode,
    AGENTPROXY_CONTRACT,
    AGENTPROXY_PROVIDER,
    WALLET_PRIVATE_KEY,
    extract_setting_digests,
    generate_signature_proof,
    Metadata,
    Proofs,
};

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    /*==========read function==========*/
    
    let agent_proxy_node = AgentProxyNode::new(
        AGENTPROXY_PROVIDER.as_str(),
        AGENTPROXY_CONTRACT.as_str(),
        WALLET_PRIVATE_KEY.as_str(),
    ).await;

    let agent_factory_address = match agent_proxy_node.get_agent_factory().await {
        Ok(agent_factory) => {
            info!("AGENT_FACTORY: {}", agent_factory);
            agent_factory
        },
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    let agent_manager_address = match agent_proxy_node.get_agent_manager().await {
        Ok(agent_manager) => {
            info!("AGENT_MANAGER: {}", agent_manager);
            agent_manager
        },
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };

    match agent_proxy_node.get_owner().await {
        Ok(owner) => info!("OWNER: {}", owner),
        Err(e) => error!("Error: {}", e),
    }

    match agent_proxy_node.get_type_and_version().await {
        Ok(type_and_version) => info!("TYPE_AND_VERSION: {}", type_and_version),
        Err(e) => error!("Error: {}", e),
    }

    let agent_manager_node = AgentManagerNode::new(
        AGENTPROXY_PROVIDER.as_str(),
        agent_manager_address.as_str(),
        WALLET_PRIVATE_KEY.as_str(),
    ).await;

    match agent_manager_node.get_owner().await {
        Ok(owner) => info!("OWNER: {}", owner),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_type_and_version().await {
        Ok(type_and_version) => info!("TYPE_AND_VERSION: {}", type_and_version),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.allowed_agent("0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b").await {
        Ok(allowed) => info!("ALLOWED_AGENT: {}", allowed),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.allowed_signer(
        "0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b",
        "0x0100e5428f61995ca2f61d96b24d90b48de58b818cc91dbb88c1bf74e83df3cb",
        "0x9538e13c0e111c5b0525f1592079aa1586B4e9Cc"
    ).await {
        Ok(allowed) => info!("ALLOWED_SIGNER: {}", allowed),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_agent_config(
        "0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b",
        "0x0100e5428f61995ca2f61d96b24d90b48de58b818cc91dbb88c1bf74e83df3cb"
    ).await {
        Ok(config) => info!("AGENT_CONFIG: {}", config),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_agent_configs(
        "0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b"
    ).await {
        Ok(configs) => {
            info!("AGENT_CONFIGS: {:?}", configs);
            let setting_digests = extract_setting_digests(&configs);
            info!("SETTING_DIGESTS: {:?}", setting_digests);
        },
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_agent_configs_count("0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b").await {
        Ok(count) => info!("AGENT_CONFIGS_COUNT: {}", count),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_agent_configs_in_range(
        "0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b",
        0,
        0
    ).await {
        Ok(configs) => info!("AGENT_CONFIGS_IN_RANGE: {}", configs),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_all_allowed_agents().await {
        Ok(agents) => info!("ALL_ALLOWED_AGENTS: {}", agents),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_all_registering_agents().await {
        Ok(agents) => info!("ALL_REGISTERING_AGENTS: {}", agents),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_allowed_agents_count().await {
        Ok(count) => info!("ALLOWED_AGENTS_COUNT: {}", count),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_allowed_agents_in_range(0, 1).await {
        Ok(agents) => info!("ALLOWED_AGENTS_IN_RANGE: {}", agents),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_registering_agents_count().await {
        Ok(count) => info!("REGISTERING_AGENTS_COUNT: {}", count),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.get_registering_agents_in_range(0, 0).await {
        Ok(agents) => info!("REGISTERING_AGENTS_IN_RANGE: {}", agents),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.is_valid_message_id("1ceb55b2-c82a-45a1-997b-ed185f8b41d6").await {
        Ok(valid) => info!("IS_VALID_MESSAGE_ID: {}", valid),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.is_valid_source_agent_id("1ceb55b2-c82a-45a1-997b-ed185f8b41d6").await {
        Ok(valid) => info!("IS_VALID_SOURCE_AGENT_ID: {}", valid),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.signer_threshold(
        "0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b",
        "0x0100e5428f61995ca2f61d96b24d90b48de58b818cc91dbb88c1bf74e83df3cb"
    ).await {
        Ok(threshold) => info!("SIGNER_THRESHOLD: {}", threshold),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.validate_data_conversion(
        "0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b",
        "0x12345455"
    ).await {
        Ok(converted_data) => info!("CONVERTED_DATA: {}", converted_data),
        Err(e) => error!("Error: {}", e),
    }

    match agent_manager_node.agent_proxy().await {
        Ok(agent_proxy) => info!("AGENT_PROXY: {}", agent_proxy),
        Err(e) => {
            error!("Error: {}", e);
        }
    }

    let agent_factory_node = AgentFactoryNode::new(
        AGENTPROXY_PROVIDER.as_str(),
        agent_factory_address.as_str(),
        WALLET_PRIVATE_KEY.as_str(),
    ).await;

    match agent_factory_node.get_agent_manager().await {
        Ok(agent_manager) => info!("AGENT_MANAGER: {}", agent_manager),
        Err(e) => error!("Error: {}", e),
    }

    match agent_factory_node.get_agent_proxy().await {
        Ok(agent_proxy) => info!("AGENT_PROXY: {}", agent_proxy),
        Err(e) => error!("Error: {}", e),
    }

    match agent_factory_node.get_agents_count().await {
        Ok(count) => info!("AGENTS_COUNT: {}", count),
        Err(e) => error!("Error: {}", e),
    }

    match agent_factory_node.get_agents_in_range(0, 0).await {
        Ok(agents) => info!("AGENTS_IN_RANGE: {}", agents),
        Err(e) => error!("Error: {}", e),
    }

    match agent_factory_node.get_all_agents().await {
        Ok(agents) => info!("ALL_AGENTS: {}", agents),
        Err(e) => error!("Error: {}", e),
    }

    match agent_factory_node.has_agent("0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b").await {
        Ok(has_agent) => info!("HAS_AGENT: {}", has_agent),
        Err(e) => error!("Error: {}", e),
    }

    match agent_factory_node.type_and_version().await {
        Ok(type_and_version) => info!("TYPE_AND_VERSION: {}", type_and_version),
        Err(e) => error!("Error: {}", e),
    }

    /*==========write function==========*/

    /*==========create and register agent==========*/

    //you should set your own parameters
    match agent_proxy_node.create_and_register_agent(
        vec!["0x9538e13c0e111c5b0525f1592079aa1586b4e9cc", "0x83390ef6B20a29ccbF0955567556AF519E86a958"], // Signer Address
        2, // Threshold
        "0x0000000000000000000000000000000000000000", // Converter Address
        "1.0", // Version
        "48b024e9-203f-4603-83bc-b925887cdde7", // Message ID
        "48b024e9-203f-4603-83bc-b925887cdde7", // Source Agent ID
        "SourceAgent", // Source Agent Name
        "48b024e9-203f-4603-83bc-b925887cdde7", // Target Agent ID
        1700000000, // Timestamp
        0, // Message Type
        1, // Priority
        3600, // TTL
    ).await {
        Ok(receipt) => info!("Agent created and registered. Receipt: {}", receipt),
        Err(e) => error!("Error: {}", e),
    }


    /*==========verify==========*/

    let message = "hello world";
    let signer_private_key_1 = env::var("SIGNER_PRIVATE_KEY_1").expect("SIGNER_PRIVATE_KEY_1 not set");
    let signer_private_key_2 = env::var("SIGNER_PRIVATE_KEY_2").expect("SIGNER_PRIVATE_KEY_2 not set");

    let private_keys: Vec<&str> = vec![
        &signer_private_key_1,
        &signer_private_key_2,
    ];
    let agent = "0xf5F190a711d1c14eBD481f37C1C0F25B79c1a14b";
    let settings_digest = "0x0100e5428f61995ca2f61d96b24d90b48de58b818cc91dbb88c1bf74e83df3cb";


    let signature_proof = generate_signature_proof(message, private_keys.clone()).await.unwrap();
    let data = hex::encode(message);
    let data_hash = ethers::utils::keccak256(message.as_bytes());

    let signature_proof_cleaned = if signature_proof.starts_with("0x") {
        &signature_proof[2..]
    } else {
        signature_proof.as_str()
    };

    let proofs = Proofs {
        signature_proof: Bytes::from(hex::decode(signature_proof_cleaned).unwrap()),
        zk_proof: Bytes::from(vec![]),
        merkle_proof: Bytes::from(vec![]),
    };

    let signature_proof_str = hex::encode(&proofs.signature_proof);
    let zk_proof_str = hex::encode(&proofs.zk_proof);
    let merkle_proof_str = hex::encode(&proofs.merkle_proof);

    let metadata = Metadata {
        content_type: "0x".to_string(),
        encoding: "0x".to_string(),
        compression: "0x".to_string(),
    };

    match agent_proxy_node.verify(
        agent,
        settings_digest,
        &data,
        &hex::encode(data_hash),
        &signature_proof_str,
        &zk_proof_str,
        &merkle_proof_str,
        &metadata.content_type,
        &metadata.encoding,
        &metadata.compression,
    ).await {
        Ok(receipt) => info!("Verify transaction successful. Receipt: {}", receipt),
        Err(e) => error!("Error calling verify function: {}", e),
    }


}

