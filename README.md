OpenArb – ICP DeFi Trading Bot

OpenArb is a decentralized trading bot built on the Internet Computer (ICP).
It automates DeFi trading with real-time blockchain analysis, on-chain execution, and ckBTC–ICP integration for seamless cross-chain arbitrage.

Overview

OpenArb provides a full DeFi trading experience in both live and demo environments.

Real-time on-chain price scanning

Automated arbitrage execution

Secure Plug wallet integration

ckBTC and ICP cross-chain trading

Non-custodial architecture

Web dashboard with live analytics

Features
Connect Plug Wallet

Connect your Plug Wallet for live blockchain trading.
Requires ICP balance and trading experience.

Create Demo Wallet

Use demo mode to safely simulate trading.
Recommended for beginners learning DeFi mechanics.

Import Demo Wallet

Continue using your previously created demo wallet.

BTC + ICP Integration

OpenArb supports ckBTC and ICP trading pairs, enabling true cross-chain price discovery and arbitrage within the ICP ecosystem.

Live Demo

You can access and interact with the live version of OpenArb here:
https://l6hb6-xaaaa-aaaak-qunea-cai.icp0.io/

Project Structure
openarb/
├── src/
│   ├── cybersec_ai_agent_backend/     # Rust backend canister (arbitrage logic, price scanning)
│   ├── wallet_canister/               # Wallet control, authentication, and access management
│   ├── openarb_frontend/              # React + Vite frontend (dashboard and trading UI)
├── dfx.json                           # Canister configuration
├── README.md

Getting Started
1. Install Dependencies

Ensure the DFINITY SDK and Node.js are installed.

sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
npm install

2. Start the Local Replica

Run a local ICP environment for development and testing:

dfx start --background

3. Deploy the Canisters

Deploy all OpenArb canisters locally:

dfx deploy
Tech Stack

Frontend: React, Vite, TailwindCSS

Backend: Rust (ICP Canisters)

Blockchain: Internet Computer Protocol (ICP)

Integration: Plug Wallet, ckBTC
