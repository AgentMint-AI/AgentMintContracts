# Cross-Chain Token Factory Contracts

This repository contains smart contracts for creating and managing tokens across multiple blockchain networks including Ethereum (EVM), Secret Network, and Solana.

> **Note:** Currently only the EVM factory contract is implemented. Secret Network and Solana factory contracts are on the roadmap for future development. Token details for Secret Network and Solana tokens are stored in the EVM factory contract (see `Erc20TokenFactory.sol` for implementation details).

## Overview

The project implements token factory contracts that allow users to create and manage tokens across different blockchain networks. It includes:

- EVM Token Factory (Ethereum/EVM chains)
- Secret Network Token Factory (SNIP-20 implementation) - Coming Soon
- Solana Token Factory - Coming Soon

## Contract Architecture

### EVM Contracts

Located in `Contracts/Evm/`:

1. **AgentRegistry.sol**
   - Manages agent registration and details
   - Stores agent information in a structured format
   - Provides functionality for agent management

2. **Erc20TokenFactory.sol**
   - Creates new ERC20 tokens
   - Manages token showcases
   - Handles token metadata
   - Supports pagination for token listings
   - Stores metadata for Secret Network and Solana tokens

3. **Erc20MemeCoin.sol**
   - Implementation of ERC20 token standard
   - Includes burning capabilities
   - Supports permit functionality

### Secret Network Contracts

Located in `Contracts/SecretNetwork/`:

- SNIP-20 reference implementation
- Privacy-preserving token standard
- Supports viewing keys and permits
- Features:
  - Configurable total supply visibility
  - Deposit/Redeem functionality
  - Minting capabilities
  - Burning functionality

### Solana Contracts

Located in `Contracts/Solana/`:

- Meme coin factory implementation
- Uses Anchor framework
- Supports SPL token creation
- Metadata management through Metaplex

## Development Setup

### EVM Contracts

Requirements:
- Foundry
- Solidity ^0.8.13
- OpenZeppelin contracts

### Secret Network

Requirements:
- Rust
- Secret Network development environment
- CosmWasm

### Solana

Requirements:
- Anchor Framework
- Solana CLI
- Node.js
- Yarn

## Testing

Each contract suite includes comprehensive tests:

- EVM: Uses Forge for testing (`forge test`)
- Secret Network: Rust unit tests and integration tests
- Solana: Anchor testing framework

## Security

- All contracts implement standard security practices
- Secret Network contracts include additional privacy features
- Regular audits recommended before production deployment

## License

- EVM Contracts: MIT License
- Secret Network Contracts: Apache License 2.0
- Solana Contracts: Apache License 2.0

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## Disclaimer

These contracts are provided as-is. Please conduct thorough testing and auditing before any production use.
