# 🐋 Mempool Monitor

A specialized monitoring tool that peers into the Ethereum txpool to analyze pending transactions before they are mined. It captures critical data like gas prices and target contracts, providing a competitive edge for researchers. Its low-latency architecture ensures high-fidelity data capture, which is vital for real-time network analysis.

## Features

- Connects to an Ethereum node via **WebSocket** (WSS)
- Subscribes to the `pendingTransactions` stream
- Identifies and logs transactions with value > **1 ETH**
- Async task spawning for non-blocking transaction inspection

## Setup

```bash
cp .env.example .env
# Add your WebSocket RPC URL (Alchemy, Infura, etc.)
```

> **Note:** Public WebSocket endpoints may be rate-limited or unstable. For best results, use a dedicated provider like [Alchemy](https://www.alchemy.com/) or [Infura](https://infura.io/).

## Usage

```bash
cargo run
```

### Sample Output

```
Connecting to WebSocket: wss://eth-mainnet.g.alchemy.com/v2/YOUR_KEY ...
Connected successfully! Monitoring pending transactions...
Listening for transactions with value > 1.0 ETH...
🐋 Large Tx -> Hash: 0xabc...def | Value: 5.2 ETH | From: 0x123...456 | To: 0x789...012
🐋 Large Tx -> Hash: 0xfed...cba | Value: 100.0 ETH | From: 0x456...789 | To: 0xabc...def
```

## Tech Stack

- `ethers` (with `ws` feature) — WebSocket provider, transaction parsing
- `tokio` — Async runtime with task spawning
- `dotenv` — Environment configuration
- `serde_json` — JSON handling
