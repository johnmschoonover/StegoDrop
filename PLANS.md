# PLANS.md - Phase 1 Implementation

## Phase 1: The Engine (The Math)

**Goal:** Implement the core cryptographic and steganographic logic in Rust, validated by a CLI.

### 1. Setup
- [x] Initialize `core` Rust project.
- [x] Configure dependencies (`pqc_kyber`, `aes-gcm`, `image`, `clap`).

### 2. Cryptography (`src/crypto.rs`)
- [x] **Kyber-768**:
  - [x] Implement `generate_keypair() -> (PublicKey, SecretKey)`.
  - [x] Implement `encapsulate(PublicKey) -> (Ciphertext, SharedSecret)`.
  - [x] Implement `decapsulate(Ciphertext, SecretKey) -> SharedSecret`.
- [x] **AES-256-GCM**:
  - [x] Implement `encrypt(key, plaintext) -> (nonce, ciphertext)`.
  - [x] Implement `decrypt(key, nonce, ciphertext) -> plaintext`.

### 3. Steganography (`src/stego.rs`)
- [x] **DCT Transform**:
  - [x] Implement 8x8 block splitting.
  - [x] Implement Forward DCT.
  - [x] Implement Inverse DCT (IDCT).
- [x] **Embedding (Mid-Band)**:
  - [x] Select coefficients (e.g., (4,3), (5,2)... middle frequencies).
  - [x] Embed bits by modifying coefficients.
- [x] **Image Handling**:
  - [x] Load image -> Convert to Channel (Lum/Y) -> Embed -> Save.

### 4. Validation (CLI)
- [x] Implement `cli` binary.
- [x] Commands:
  - `keygen`: Generates Kyber identity.
  - `encap`: Simulates User B creating a shared secret.
  - `decap`: Simulates User A recovering the shared secret.
  - `embed`: Encrypts & embeds message into image.
  - `extract`: Extracts & decrypts message from image.

### 5. Verification
- [x] Run full "Alice & Bob" flow using the CLI.
- [x] Ensure `cargo test` passes.
