K社Businessモデル検証用Demo Repository

Goal
- Reach 1K TPS on peak transaction throughput during batch minting of NFT.

Method
- We have written a simple NFT contract with two properties: index and a random number.
- We then use hardhat script to simulate a single client with negligible geographic and networking overhead to repeatedly send the same transaction over and over to the sequencer.

Outcome
- Hardhat Runtime Environment: 120~140TPS
- Arbitrum Nitro Geth Node: 260~280TPS
- Arbitrum Stylus w/ Rust Code: TBD
- Arbitrum Stylus w/ Rust Code and Orbit: TBD