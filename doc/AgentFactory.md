# Agent Factory SDK Documentation

## Overview
**AgentFactoryNode** is a Rust module designed for interacting with the **AgentFactory smart contract**. It provides a variety of functionalities for managing and interacting with agents, including:
- Deploying new agents
- Retrieving agent information
- Managing collections of agents

---

## Functions

### 1. `new`
**Purpose**: Initialize a new `AgentFactoryNode` instance.  
**Parameters**:
- `provider_url`: Ethereum provider URL.
- `contract_addr`: Address of the AgentFactory contract.
- `private_key`: Private key used to sign transactions.

**Description**: Sets up the contract instance using the specified Ethereum provider and wallet.

---

### 2. `get_agent_proxy`
**Purpose**: Retrieve the associated AgentProxy address.  
**Returns**: A string representation of the AgentProxy address.  
**Description**: Calls the `agentProxy` function on the `AgentFactory` contract to fetch the proxy address.

---

### 3. `get_agents_count`
**Purpose**: Retrieve the total number of registered agents.  
**Returns**: An unsigned integer representing the number of agents.  
**Description**: Calls the `getAgentsCount` function on the `AgentFactory` contract.

---

### 4. `get_agents_in_range`
**Purpose**: Retrieve a list of agent addresses within a specified range.  
**Parameters**:
- `agent_idx_start`: Starting index of the agent range.
- `agent_idx_end`: Ending index of the agent range.

**Returns**: A string representation of the list of agent addresses.  
**Description**: Calls the `getAgentsInRange` function on the `AgentFactory` contract.

---

### 5. `get_all_agents`
**Purpose**: Retrieve all registered agent addresses.  
**Returns**: A string representation of the list of all agent addresses.  
**Description**: Calls the `getAllAgents` function on the `AgentFactory` contract.

---

### 6. `has_agent`
**Purpose**: Check if a specified agent exists.  
**Parameters**:
- `agent`: Address of the agent.

**Returns**: A boolean indicating whether the agent exists.  
**Description**: Calls the `hasAgent` function on the `AgentFactory` contract.

---

### 7. `type_and_version`
**Purpose**: Retrieve the type and version of the `AgentFactory`.  
**Returns**: A string representing the type and version.  
**Description**: Calls the `typeAndVersion` function on the `AgentFactory` contract.
