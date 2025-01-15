# ATTPS RUST SDK

## Overview
The **ATTPS RUST SDK** is a Rust-based software development kit (SDK) designed for interacting with smart contracts on Ethereum. It focuses on three core contracts: **AgentFactory**, **AgentManager**, and **AgentProxy**. Each contract is associated with a dedicated Rust module, providing functionalities to manage and operate agents.


## Quickstart: ATTPS RUST SDK

This guide will help you quickly set up and use the ATTPS RUST SDK to interact with Ethereum smart contracts.

---

## Step 1: Clone the Repository
Clone the ATTPS RUST SDK repository from GitHub:
```bash
git clone https://github.com/anz-io/attps-rust-sdk.git
cd attps-rust-sdk/demo
```

## Step 2: Install Rust Environment
Ensure you have Rust installed. The recommended Rust version is 1.83.0. Check your Rust version with:

```bash
rustc -V
```

Expected output:

```bash
rustc 1.83.0 (90b35a623 2024-11-26)
```

## Step 3: Configure Environment Variables
Copy the example environment file and update it with your settings:

```bash
cp .env.example .env
```

Edit .env and set the following values:

```bash
AGENTPROXY_CONTRACT=0xAgentProxyContractAddress
AGENTPROXY_PROVIDER=yourRPClink
WALLET_PRIVATE_KEY=your_private_key
SIGNER_PRIVATE_KEY_1=signer_private_key_1
SIGNER_PRIVATE_KEY_2=signer_private_key_2
##you can add more signer private key in the .env file
```

## Step 4: Customize the main Function
Open the main function in your codebase and replace the parameters with your specific values. For example, when creating and registering an agent:

```bash
match agent_proxy_node.create_and_register_agent(
    vec!["0x9538e13c0e111c5b0525f1592079aa1586b4e9cc", "0x83390ef6B20a29ccbF0955567556AF519E86a958"], // Signer Addresses
    2, // Threshold
    "0x0000000000000000000000000000000000000000", // Converter Address
    "1.0", // Version
    "580ac1b7-eeb1-4352-9e26-b24ad7aa022e", // Message ID
    "580ac1b7-eeb1-4352-9e26-b24ad7aa022e", // Source Agent ID
    "SourceAgent", // Source Agent Name
    "580ac1b7-eeb1-4352-9e26-b24ad7aa022e", // Target Agent ID
    1700000000, // Timestamp
    0, // Message Type
    1, // Priority
    3600, // TTL
).await {
    Ok(_) => println!("Agent created and registered"),
    Err(e) => println!("Error: {}", e),
}
```

You can comment out any unused function calls to keep the output clean.

## Step 5: Run the Project
Run the project using Cargo:

```bash
cargo run
```

## Example Output

If successful, you will see transaction details including gas usage and transaction receipt:

```bash
Gas estimate: 1513376
Transaction successful: TransactionReceipt {
    transaction_hash: 0x460524c261131045ec1fee06a45e40e019c48daa8c9ffcc00de832b7b0b5b0d3,
    transaction_index: 12,
    block_hash: Some(0x7df911943a504190a3e2d8043214d382053ee8e1a1c18b1ef7d1e3150ef0bd05),
    block_number: Some(7486998),
    from: 0x9538e13c0e111c5b0525f1592079aa1586b4e9cc,
    to: Some(0x590cde19ee0fba69a72d37d1b8c3474f327c8eec),
    gas_used: Some(1500302),
    ...
}
Agent created and registered
```

Verify transactions may show additional details:

```bash
Gas estimate: 154216
Verify transaction successful. Receipt: TransactionReceipt {
    transaction_hash: 0x7297107c5ecfae5aa1f02369b8d05ae067bb50570b5e273ba538184f9a594476,
    gas_used: Some(151844),
    ...
}
```

## Conclusion
The ATTPS RUST SDK is now set up and ready for use. You can interact with Ethereum smart contracts by customizing the provided API functions and running them to deploy, manage, and verify agents.


---

## Project Modules

### 1. **AgentFactoryNode**
**Purpose**: Facilitates interaction with the **AgentFactory contract**.  
**Key Features**:
- Deploying new agents
- Retrieving agent information
- Managing collections of agents

**Related Functions**:
- Retrieve the address of the agent factory
- Get the total number of agents
- Retrieve agents within a specific range

---

### 2. **AgentManagerNode**
**Purpose**: Facilitates interaction with the **AgentManager contract**.  
**Key Features**:
- Retrieving contract information
- Managing agent configurations
- Verifying agent permissions

**Related Functions**:
- Retrieve the address of the agent manager
- Check if agents and signers are permitted
- Retrieve agent configurations

---

### 3. **AgentProxyNode**
**Purpose**: Facilitates interaction with the **AgentProxy contract**.  
**Key Features**:
- Retrieving contract information
- Managing contract ownership
- Handling agent configurations

**Related Functions**:
- Retrieve the addresses of the agent factory and agent manager
- Get the owner of the contract
- Accept or transfer contract ownership

---

## Functional Overview

### 1. **Contract Initialization**
Allows initializing contract instances by providing:
- Ethereum provider URL
- Contract address
- Private key for signing transactions

### 2. **Information Retrieval**
Provides methods to fetch various contract-related details, such as:
- Agent factory address
- Agent manager address
- Contract owner

### 3. **Agent Management**
Includes methods for managing agent configurations and permissions:
- Validate agent permissions
- Create and register new agents
- Modify existing agent settings

### 4. **Transaction Handling**
Offers tools to handle smart contract transactions:
- Estimate gas
- Send transactions
- Process transaction receipts

---

## Use Cases

### 1. **Agent Deployment and Management**
Ideal for scenarios requiring the deployment and management of multiple agents on Ethereum.

### 2. **Contract Interaction**
Equipped with a comprehensive API to facilitate complex interactions with smart contracts.

### 3. **Permission Validation**
Provides functionality to verify the permissions of agents and signers, making it suitable for scenarios with strict access controls.

You can find more description about the functions in the [AgentFactory.md](./doc/AgentFactory.md), [AgentManager.md](./doc/AgentManager.md), [AgentProxy.md](./doc/AgentProxy.md).

