# 🦀 Kaushal’s Wholesome Rust Repository

Welcome to my curated Rust journey!  
This repository tracks all the Rust-based projects I’m building — from games to blockchain smart contracts — documented with learnings and progress over time.

---

## 🗂️ Projects

### 📍 1. `bounce.rs` – Terminal Bouncing Ball Game  
📅 **Created:** June 20, 2025  
🔧 **Tech:** Pure Rust, Terminal UI  
🎮 **Description:**  
A minimal bouncing ball animation game built in the terminal using Rust — no libraries, just core logic.  
This project taught me about:
- Ownership and borrowing in Rust
- Pattern matching and enums
- Structs, method dispatch, and display formatting
- Building a custom game loop

📂 **Source:** [`bounce.rs`](./projects/bounce.rs)

---

### 📍 2. `stellar-marketplace` – On-chain Marketplace (Soroban)  
📅 **Started:** June 21, 2025  
🔧 **Tech:** Rust, Stellar Soroban SDK  
💸 **Description:**  
A smart contract powered marketplace deployed on the Stellar blockchain using Soroban.  
Supports:
- Product registration (id,name, price, stock_quantity)
- Token-based payment logic
- Time-bound claimable balances
- Chainlink Automation (coming soon)

🧠 This project focuses on:
- Writing secure no-std Rust contracts
- Storage patterns (`DataKey`, `instance()` etc.)
- On-chain/off-chain component design
- Integration with token standards

###📍 3. `token-contracts` – Custom Token Standard (Soroban)
📅 Started: June 24, 2025
🔧 Tech: Rust, Stellar Soroban SDK
🔐 Description:
A modular token contract supporting minting, burning, transfers, approvals, and account freezing.
Key components:

Admin-controlled operations (mint, burn, freeze, unfreeze)
ERC20-style allowance with expiration
Instance TTL management for storage safety
Metadata handling (name, symbol, decimals)
Event emission for all critical token actions
📂 Source: token-contracts
🔍 Explorer: [Contract on Stellar Testnet](https://stellar.expert/explorer/testnet/contract/CBEZZYS7KNL7PO7BDGRD2WJAODRB67HM65BO2HVMO44SBXEDVRPE33SU)

## 🧠 Learnings & Takeaways

Each project folder contains:
- Source code
- Key learnings (`LEARNINGS.md`)
- Test cases and simulation (if any)

---

## 📌 Goals

- Build 10+ projects in Rust (Web2 + Web3)
- Master smart contract development using Soroban
- Build an open on-chain UI component library (in progress)
- Share learnings in the form of X (Twitter) threads & blog posts

---

## 📅 Timeline

| Date       | Project               | Summary                                  | Status | Deployed Contract                                                                                                            |
| ---------- | --------------------- | ---------------------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------- |
| 2025-06-20 | `bounce.rs`           | Terminal game with physics logic         | Done   | Not a contract                                                                                                               |
| 2025-06-21 | `stellar-marketplace` | Marketplace contract on Soroban          | Done   | [CBIG4CP6P2FN...](https://stellar.expert/explorer/testnet/contract/CBIG4CP6P2FNCPNA444KYZOLDRISVVVHARDHDDK3IZB5T6HSRUUXJB6V) |
| 2025-06-24 | `token-contracts`     | Modular token contract with freeze logic | Done   | [CBEZZYS7KNL7...](https://stellar.expert/explorer/testnet/contract/CBEZZYS7KNL7PO7BDGRD2WJAODRB67HM65BO2HVMO44SBXEDVRPE33SU) |

---

## 📬 Feedback / Collaboration

Feel free to open issues, suggest improvements, or collaborate on anything Rust-related.  
Rustaceans welcome! 🦀

---
