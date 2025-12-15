# **📦 StegoDrop: The Anti-Metadata PQC Utility**

StegoDrop is a cross-platform (iOS/Android) utility designed for covert communication, built from the ground up to defeat the "Harvest Now, Decrypt Later" quantum threat and eliminate transport metadata.

It operates entirely locally, is serverless, and functions solely as a tool to prepare a file for distribution via any public channel (social media, email, forums).

### **🛡️ Why StegoDrop?**

Traditional secure messengers protect message *content* but expose the *metadata* (who, when, how often). StegoDrop addresses both layers of vulnerability:

1. **Quantum Resistance:** Uses **Kyber-768** (Post-Quantum Key Encapsulation) for secure key exchange, future-proofing content against quantum decryption attacks.  
2. **Stealth:** Uses **DCT Mid-Band Steganography** to embed AES-256 encrypted payloads into innocuous image files, ensuring that the network logs see only a generic image, not an encrypted communication.

### **✨ Core Features**

* **PQC Handshake:** Secure key exchange via QR code using **Kyber-768**.  
* **Anti-Metadata:** Eliminates all network-level communication metadata.  
* **Anti-Forensics:** Includes mandatory **EXIF stripping** and optional **Ghost Mode** (RAM-only message history, no local contact aliases).  
* **Anti-Replay Defense:** Uses a strict monotonic counter to reject out-of-order or replayed messages, securing the key ratchet state.  
* **Licensing:** Open-source core under the **MIT License**.

### **🛠️ Architecture Overview**

The project uses a Monorepo structure containing platform-specific implementations (Swift/Kotlin) that wrap a shared cryptographic and steganography engine.

| Component | Technology | Role |
| :---- | :---- | :---- |
| **PQC** | Kyber-768 | Key Encapsulation (Layer 1\) |
| **Encryption** | AES-256-GCM | Message Payload Encryption (Layer 2\) |
| **Steganography** | DCT Mid-Band | JPEG Resilience |
| **License** | MIT | Core Transparency & Auditability |

License: [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)]
PQC Algorithm: [![PQC: Kyber-768](https://img.shields.io/badge/PQC-Kyber--768-333333)]

Status: [![Status: Active Development](https://img.shields.io/badge/Status-Active%20Development-blue)]
