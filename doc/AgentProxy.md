# Agent Proxy SDK Documentation

## Overview
**AgentProxyNode** is a Rust module designed for interacting with the **AgentProxy smart contract**. It provides a variety of functionalities to manage and interact with agents, including:
- Retrieving contract information
- Managing ownership
- Handling agent configurations

---

## Functions

### 1. `new`
**Purpose**: Initialize a new `AgentProxyNode` instance.  
**Parameters**:
- `provider_url`: Ethereum provider URL.
- `contract_addr`: Address of the AgentProxy contract.
- `private_key`: Private key used to sign transactions.

**Description**: Sets up the contract instance using the specified Ethereum provider and wallet.

---

### 2. `get_agent_factory`
**Purpose**: Retrieve the associated AgentFactory address.  
**Returns**: A string representation of the AgentFactory address.  
**Description**: Calls the `agent_factory` function on the `AgentProxy` contract to retrieve the factory address.

---

### 3. `get_agent_manager`
**Purpose**: Retrieve the associated AgentManager address.  
**Returns**: A string representation of the AgentManager address.  
**Description**: Calls the `agent_manager` function on the `AgentProxy` contract to retrieve the manager address.

---

### 4. `get_owner`
**Purpose**: Retrieve the current owner of the `AgentProxy`.  
**Returns**: A string representation of the owner's address.  
**Description**: Calls the `owner` function on the `AgentProxy` contract to fetch the owner's address.

---

### 5. `get_type_and_version`
**Purpose**: Retrieve the type and version of the `AgentProxy`.  
**Returns**: A string representing the type and version.  
**Description**: Calls the `type_and_version` function on the `AgentProxy` contract.

---

### 6. `accept_ownership`
**Purpose**: Accept ownership of the `AgentProxy`.  
**Description**: Sends a transaction to call the `accept_ownership` function on the `AgentProxy` contract. It estimates the required gas and processes the transaction receipt.

---

### 7. `set_agent_factory`
**Purpose**: Set a new AgentFactory address.  
**Parameters**:
- `factory_address`: The new AgentFactory address.

**Description**: Sends a transaction to call the `set_agent_factory` function on the `AgentProxy` contract. It estimates the required gas and processes the transaction receipt.

---

### 8. `set_agent_manager`
**Purpose**: Set a new AgentManager address.  
**Parameters**:
- `manager_address`: The new AgentManager address.

**Description**: Sends a transaction to call the `set_agent_manager` function on the `AgentProxy` contract. It estimates the required gas and processes the transaction receipt.

---

### 9. `transfer_ownership`
**Purpose**: Transfer ownership to a new address.  
**Parameters**:
- `new_owner_address`: The address of the new owner.

**Description**: Sends a transaction to call the `transfer_ownership` function on the `AgentProxy` contract. It estimates the required gas and processes the transaction receipt.

---

### 10. `create_and_register_agent`
**Purpose**: Create and register a new agent.  
**Parameters**:
- `signers`: List of signer addresses.
- `threshold`: Threshold for the agent.
- `converter_address`: Converter address.
- `version`: Agent version.
- `message_id`: Message ID.
- `source_agent_id`: Source agent ID.
- `source_agent_name`: Source agent name.
- `target_agent_id`: Target agent ID.
- `timestamp`: Timestamp.
- `message_type`: Message type.
- `priority`: Priority.
- `ttl`: Time-to-live.

**Description**: Constructs agent settings and sends a transaction to call the `create_and_register_agent` function on the `AgentProxy` contract. It estimates the required gas and processes the transaction receipt.

---

### 11. `verify`
**Purpose**: Verify agent data.  
**Parameters**:
- `agent`: Address of the agent.
- `settings_digest`: Digest of the settings.
- `data`: Data to be verified.
- `data_hash`: Hash of the data.
- `signature_proof`: Signature proof.
- `zk_proof`: Zero-knowledge proof.
- `merkle_proof`: Merkle proof.
- `content_type`: Content type of the data.
- `encoding`: Encoding of the data.
- `compression`: Compression method used.

**Description**: Constructs the message payload and sends a transaction to call the `verify` function on the `AgentProxy` contract. It estimates the required gas and processes the transaction receipt.
