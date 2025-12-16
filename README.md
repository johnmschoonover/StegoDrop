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

---

### **✅ Phase 1 Validation (The Engine)**

To validate that Phase 1 (Core Crypto & Stego Engine) is working correctly, use the provided CLI tool in `core/`.

**Prerequisites:**
- Rust installed (`cargo`).

**Steps:**

1.  **Build the Core:**
    ```bash
    cd core
    cargo build --release
    ```

2.  **Run the Validation Flow:**
    You can simulate a full message exchange between Alice and Bob.

    *   **Step 1: Alice generates her Identity (Kyber-768)**
        ```bash
        cargo run --bin cli -- keygen --out alice_identity
        # Outputs: alice_identity.pub, alice_identity.sec
        ```

    *   **Step 2: Bob encapsulates a Shared Secret for Alice**
        ```bash
        cargo run --bin cli -- encap --pub-key alice_identity.pub --out bob_secret
        # Outputs: bob_secret.shared (Secret), bob_secret.cipher (Ciphertext to send to Alice)
        ```

    *   **Step 3: Alice decapsulates to get the same Shared Secret**
        ```bash
        cargo run --bin cli -- decap --sec-key alice_identity.sec --cipher bob_secret.cipher --out alice_secret
        # Outputs: alice_secret.shared (Should match bob_secret.shared)
        ```

    *   **Step 4: Bob sends a Covert Message**
        Bob embeds a message into an image using the Shared Secret.
        ```bash
        cargo run --bin cli -- embed --image input.png --secret bob_secret.shared --message "Hello Quantum World" --out stego.png
        ```

    *   **Step 5: Alice receives and reads the Covert Message**
        Alice uses her copy of the Shared Secret to extract the message.
        ```bash
        cargo run --bin cli -- extract --image stego.png --secret alice_secret.shared
        # Output: "Hello Quantum World"
        ```

3.  **Run Unit Tests:**
    ```bash
    cargo test
    ```
