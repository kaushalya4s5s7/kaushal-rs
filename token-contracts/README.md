# Soroban Token Contracts

## Overview

This repository implements a custom token contract for the Soroban smart contract platform. The contract provides a robust token standard with features such as minting, burning, transfer, approval, allowance, freezing/unfreezing accounts, and metadata management. The contract is modularized for clarity and maintainability.

## Project Structure

```text
.
├── contracts
│   └── Token-Contracts
│       ├── src
│       │   ├── admin.rs
│       │   ├── allowance.rs
│       │   ├── balance.rs
│       │   ├── contract.rs
│       │   ├── metadata.rs
│       │   ├── storage_types.rs
│       │   └── lib.rs
│       ├── Cargo.toml
│       └── Makefile
├── Cargo.toml
└── README.md
```

## Contract Modules

- **admin.rs**: Handles contract administrator logic, including initialization, admin updates, and access control.
- **allowance.rs**: Implements ERC20-style allowance and approval mechanisms, including reading, writing, and spending allowances.
- **balance.rs**: Manages user balances, including reading, writing, transferring, and spending balances.
- **contract.rs**: Main contract logic, exposing the token interface and implementing all core token operations.
- **metadata.rs**: Manages token metadata (name, symbol, decimals) using Soroban's metadata utilities.
- **storage_types.rs**: Defines storage keys, state enums (e.g., freeze/unfreeze), and data structures for contract storage.

## Key Features

- **Initialization**: The contract must be initialized by an admin, setting metadata and admin address.
- **Minting**: Only the admin can mint new tokens to a specified address.
- **Burning**: Tokens can be burned by the owner or by an approved spender.
- **Transfers**: Standard transfer and transfer_from (with allowance) functions are implemented.
- **Approvals & Allowances**: Users can approve spenders with an expiration ledger, and spenders can transfer tokens on behalf of owners.
- **Account Freezing**: Admin can freeze/unfreeze accounts, restricting their ability to perform actions.
- **Metadata**: Name, symbol, and decimals are stored and can be queried.
- **Events**: Emits events for transfers, approvals, and burns for off-chain tracking.

## Usage

- Deploy the contract to Soroban.
- Call `initialize` with admin address and token metadata.
- Use `mint` (admin only) to create tokens.
- Use `transfer`, `approve`, `transfer_from`, `burn`, and `burn_from` as per standard token workflows.
- Admin can use `setadmin` to change admin and `set_state`/`get_state` to freeze/unfreeze accounts.

## Example

```rust
// Initialize contract
TokenClient::initialize(&env, admin_address, 8, "TokenName".into(), "TKN".into());

// Mint tokens
TokenClient::mint(&env, user_address, 1000);

// Transfer tokens
TokenClient::transfer(&env, user_address, recipient_address, 100);

// Approve allowance
TokenClient::approve(&env, user_address, spender_address, 500, expiration_ledger);

// Transfer from (using allowance)
TokenClient::transfer_from(&env, spender_address, user_address, recipient_address, 200);
```

## Security

- All sensitive actions (mint, setadmin, freeze/unfreeze) require admin authentication.
- Negative values are checked and rejected.
- Allowance and balance operations are safe and revert on insufficient funds or permissions.


## Contract
- Contract address -> CBEZZYS7KNL7PO7BDGRD2WJAODRB67HM65BO2HVMO44SBXEDVRPE33SU
- Explorer -> https://stellar.expert/explorer/testnet/contract/CBEZZYS7KNL7PO7BDGRD2WJAODRB67HM65BO2HVMO44SBXEDVRPE33SU

## Extending

You can add new contracts to the `contracts` directory, each with its own `Cargo.toml`. The workspace is set up for easy expansion.

---

For more details, see the source files in [contracts/Token-Contracts/src](contracts/Token-Contracts/src).