# NFT-based Digital Certificate Issuance

## 📌 Project Title
**NFT-based Digital Certificate Issuance**

## 📄 Project Description
A decentralized smart contract for issuing, storing, and verifying academic or professional certificates as NFTs on the blockchain. Each certificate is uniquely tied to the recipient and tamper-proof.

## 🌟 Project Vision
To provide a secure, immutable, and verifiable way to issue and manage digital certificates using blockchain technology, eliminating fraud and simplifying verification processes.

## ✨ Key Features
- Unique NFT-style certificates tied to recipients
- On-chain certificate metadata including issuer, course, and timestamp
- Verifiable and immutable storage
- Public certificate look-up by ID or recipient

## 📜 Contract Details

### Contract Address: CA475V4CGNGYJ4XJJAL3YZHTV3ZDIAODWI67VSS6NIWTTNL3LTQTQTAN

### `issue_cert(env, issuer, recipient, cert_id, course)`
Issues a certificate from `issuer` to `recipient`. The issuer must authenticate the call. Certificates are stored with metadata including course name and issuance timestamp.

### `get_cert(env, cert_id) -> Option<Certificate>`
Fetch a certificate by its unique ID.

### `list_certs(env, recipient) -> Vec<Symbol>`
Returns a list of certificate IDs issued to a specific address.

---

🛠️ Built with [Soroban SDK](https://soroban.stellar.org/) for Stellar-based smart contracts.

Ideal for institutions, academies, and certifying bodies looking to modernize credential issuance with Web3.
