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

| Date       | Project               | Summary                         |    status  | deployed contract |
|------------|---------------------  |---------------------------------|------------|-------------------|
| 2025-06-20 | `bounce.rs`           | Terminal game with physics logic|   Done     |Not contract       |
| 2025-06-21 | `stellar-marketplace` | Marketplace contract on Soroban |   Done     |Stellar(CBIG4CP6P2FNCPNA444KYZOLDRISVVVHARDHDDK3IZB5T6HSRUUXJB6V)|

---

## 📬 Feedback / Collaboration

Feel free to open issues, suggest improvements, or collaborate on anything Rust-related.  
Rustaceans welcome! 🦀

---
