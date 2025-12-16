# AGENTS.md

## Scope & Context
This repository, `StegoDrop`, is a Monorepo for a Post-Quantum Cryptography (PQC) utility.
Current focus: **Phase 1 (The Engine)**.

## Project Structure
- `core/`: Rust crate containing the shared cryptographic and steganography engine.
  - `src/lib.rs`: The library entry point.
  - `src/crypto.rs`: Kyber-768 and AES-256-GCM implementation.
  - `src/stego.rs`: DCT Mid-Band Steganography implementation.
  - `src/bin/cli.rs`: CLI tool for validation.
- `docs/`: Project documentation (PDD, etc.).
- `PLANS.md`: Implementation roadmap.

## Development Guidelines
1.  **Language**: Rust (2021 edition).
2.  **Testing**: All core logic must be unit tested. `cargo test` must pass.
3.  **Validation**: Use the CLI to verify the full flow (KeyGen -> Exchange -> Embed -> Extract).
4.  **No Mocking of Math**: Implement actual algorithms (Kyber via `pqc_kyber`, AES via `aes-gcm`).
5.  **Steganography**: Implement DCT Mid-Band embedding. Focus on correctness of the algorithm.

## Interaction
- When making changes, ensure `PLANS.md` is updated if the roadmap changes.
- Verify all changes with `cargo test` and the CLI flow.
