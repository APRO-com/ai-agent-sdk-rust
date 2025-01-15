# Agent Manager SDK Documentation

## Overview
**AgentManagerNode** is a Rust module designed for interacting with the **AgentManager smart contract**. It provides a variety of functionalities for managing and interacting with agents, including:
- Retrieving contract information
- Managing agent configurations
- Verifying agent permissions

---

## Functions

### 1. `new`
**Purpose**: Initialize a new `AgentManagerNode` instance.  
**Parameters**:
- `provider_url`: Ethereum provider URL.
- `contract_addr`: Address of the AgentManager contract.
- `private_key`: Private key used to sign transactions.

**Description**: Sets up the contract instance using the specified Ethereum provider and wallet.

---

### 2. `get_agent_proxy`
**Purpose**: Retrieve the associated AgentProxy address.  
**Returns**: A string representation of the AgentProxy address.  
**Description**: Calls the `agentProxy` function on the `AgentManager` contract to fetch the proxy address.

---

### 3. `get_owner`
**Purpose**: Retrieve the current owner of the `AgentManager`.  
**Returns**: A string representation of the owner's address.  
**Description**: Calls the `owner` function on the `AgentManager` contract to get the owner's address.

---

### 4. `get_type_and_version`
**Purpose**: Retrieve the type and version of the `AgentManager`.  
**Returns**: A string representing the type and version.  
**Description**: Calls the `type_and_version` function on the `AgentManager` contract.

---

### 5. `set_agent_proxy`
**Purpose**: Set a new AgentProxy address.  
**Parameters**:
- `proxy_address`: The new AgentProxy address.

**Description**: Sends a transaction to call the `set_agent_proxy` function on the `AgentManager` contract. It estimates the required gas and processes the transaction receipt.

---

### 6. `allowed_agent`
**Purpose**: Check if a specified agent is allowed.  
**Parameters**:
- `agent`: Address of the agent.

**Returns**: A boolean indicating whether the agent is allowed.  
**Description**: Calls the `allowedAgent` function on the `AgentManager` contract.

---

### 7. `allowed_signer`
**Purpose**: Check if a specified signer is allowed.  
**Parameters**:
- `agent`: Address of the agent.
- `setting_digest`: Digest of the settings.
- `signer`: Address of the signer.

**Returns**: A boolean indicating whether the signer is allowed.  
**Description**: Calls the `allowedSigner` function on the `AgentManager` contract.

---

### 8. `get_agent_config`
**Purpose**: Retrieve the configuration of a specified agent.  
**Parameters**:
- `agent`: Address of the agent.
- `setting_digest`: Digest of the settings.

**Returns**: A string representation of the agent's configuration.  
**Description**: Calls the `getAgentConfig` function on the `AgentManager` contract.

---

### 9. `get_agent_configs`
**Purpose**: Retrieve configurations of all agents.  
**Parameters**:
- `agent`: Address of the agent.

**Returns**: A string representation of the list of agent configurations.  
**Description**: Calls the `getAgentConfigs` function on the `AgentManager` contract.
