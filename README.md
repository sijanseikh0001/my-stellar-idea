# my-stellar-idea
# 📍 Location Rewards – Soroban Smart Contract

## 🚀 Project Description

**Location Rewards** is a decentralized reward system built on the Stellar using the Soroban smart contract platform.

This project enables users to earn reward points by checking in at different locations. It showcases how blockchain technology can be used to create **transparent, tamper-proof, and incentive-driven location-based systems** for real-world applications like retail, tourism, and events.

<img width="1354" height="614" alt="Screenshot 2026-04-13 142944" src="https://github.com/user-attachments/assets/25c94dae-bc6d-4e31-879e-c1c88fb4c61a" />

---

## ⚙️ What it does

* Allows users to check in at different locations (represented as strings).
* Rewards users with points for each unique location visit.
* Prevents duplicate check-ins at the same location.
* Stores all user data securely on-chain.
* Enables users to view their total reward points anytime.

---

## ✨ Features

* 📍 **Location-Based Check-ins**
  Users can check in at multiple locations and earn rewards.

* 🎯 **Reward Points System**
  Each successful check-in grants a fixed number of points.

* 🚫 **Duplicate Check-in Protection**
  Prevents users from earning rewards multiple times for the same location.

* 🔐 **Secure Authentication**
  Only the wallet owner can authorize and perform transactions.

* 📊 **On-chain Storage**
  User points and visited locations are stored permanently and transparently.

* ⚡ **Efficient & Low Cost**
  Built with Soroban for fast execution and minimal transaction cost.

---

## 🛠️ Tech Stack

* **Language:** Rust
* **Smart Contract Framework:** Soroban SDK
* **Blockchain:** Stellar

---

## 📂 Smart Contract Overview

The smart contract includes the following core functions:

### `check_in(user: Address, location: String)`

* Authenticates the user
* Checks if the location is already visited
* Stores the new location
* Adds reward points to the user

### `get_points(user: Address) -> i32`

* Returns the total reward points of the user

---

## 🔗 Deployed Smart Contract Link

`XXX`

> Replace this placeholder with your deployed contract address or a link from Stellar Explorer after deployment.

---

## 💡 Use Cases

* 🏬 **Retail Loyalty Programs**
  Reward customers for visiting stores or outlets

* 🧳 **Travel & Tourism Apps**
  Encourage users to explore new places

* 🎉 **Event Check-ins**
  Track and reward attendance at events or meetups

* 🎮 **Gamified City Exploration**
  Create interactive location-based games

---

## 🔮 Future Improvements

* 📡 Integration with GPS or external oracles for real location verification
* 🏆 Leaderboard system for top users
* 🎁 NFT or token rewards for special milestones
* 💰 Reward redemption system (discounts, coupons, etc.)
* 📱 Frontend UI with wallet integration

---

## ⚙️ How to Run (Basic)

1. Install Soroban CLI
2. Build the contract:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```
3. Deploy on Stellar testnet:

   ```bash
   soroban contract deploy ...
   ```
4. Interact with functions:

   ```bash
   soroban contract invoke ...
   ```

---

## 🤝 Contribution

Contributions are welcome!
Feel free to fork the repository and submit pull requests.

---

## 📜 License

This project is licensed under the MIT License.
<img width="1354" height="614" alt="Screenshot 2026-04-13 142944" src="https://github.com/user-attachments/assets/fe62f59c-edff-4883-9966-acfe348dd061" />
