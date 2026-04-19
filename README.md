# trinos-crypto

[![Zig](https://img.shields.io/badge/Zig-0.15+-F7A41D?logo=zig&logoColor=white)](https://ziglang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Bitcoin](https://img.shields.io/badge/Bitcoin-orange)]()
[![DePIN](https://img.shields.io/badge/DePIN-blue)]()

> **Crypto primitives** — Bitcoin PoW, Proof-of-Archival-Storage, and DePIN consensus.

## Features

- ⛏️ **PoW Mining** — Bitcoin hash rate optimization
- 🗃️ **DePIN** — Proof-of-Archival-Storage consensus
- 💰 **Wallet** — Bitcoin key management
- 🕸️ **Merkle Tree** — Transaction verification
- 🔒 **Signatures** — ECDSA/Ed25519 cryptography

## Installation

\`\`\`cargo build\`\` — Build crypto executables

## Architecture

\`\`\`
src/
├── crypto/         BTC mining, PoW, DePIN
├── depin/          DePIN consensus core
└── wallet/         Bitcoin key management
\`\`

## Ecosystem

Part of trinos:
- [trinos-golden-float](https://github.com/gHashTag/trinos-golden-float) → Sacred constants
- [trinos-agents](https://github.com/gHashTag/trinos-agents) → Agent MCP integration

## License

MIT © Dmitrii Vasilev
