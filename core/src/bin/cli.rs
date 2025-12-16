use clap::{Parser, Subcommand};
use stegodrop_core::crypto::CryptoManager;
use stegodrop_core::stego::StegoEngine;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "stegodrop-cli")]
#[command(about = "CLI for StegoDrop Phase 1 Validation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a Kyber-768 keypair for identity
    Keygen {
        #[arg(short, long)]
        out: String,
    },
    /// Encapsulates a shared secret using a public key
    Encap {
        #[arg(short, long)]
        pub_key: PathBuf,
        #[arg(short, long)]
        out: String,
    },
    /// Decapsulates a shared secret using a secret key
    Decap {
        #[arg(short, long)]
        sec_key: PathBuf,
        #[arg(short, long)]
        cipher: PathBuf,
        #[arg(short, long)]
        out: String,
    },
    /// Encrypts and embeds a message into an image
    Embed {
        #[arg(short, long)]
        image: PathBuf,
        #[arg(short, long)]
        secret: PathBuf,
        #[arg(short, long)]
        message: String,
        #[arg(short, long)]
        out: PathBuf,
    },
    /// Extracts and decrypts a message from an image
    Extract {
        #[arg(short, long)]
        image: PathBuf,
        #[arg(short, long)]
        secret: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { out } => {
            println!("Generating Kyber-768 keypair...");
            let (pk, sk) = CryptoManager::generate_kyber_keypair()?;

            let mut file_pk = File::create(format!("{}.pub", out))?;
            file_pk.write_all(&pk)?;

            let mut file_sk = File::create(format!("{}.sec", out))?;
            file_sk.write_all(&sk)?;

            println!("Keys saved to {}.pub and {}.sec", out, out);
        }
        Commands::Encap { pub_key, out } => {
            let mut pk_buf = [0u8; pqc_kyber::KYBER_PUBLICKEYBYTES];
            let mut f = File::open(pub_key)?;
            f.read_exact(&mut pk_buf)?;

            println!("Encapsulating shared secret...");
            let (secret, cipher) = CryptoManager::encapsulate_secret(&pk_buf)?;

            let mut file_secret = File::create(format!("{}.shared", out))?;
            file_secret.write_all(&secret)?;

            let mut file_cipher = File::create(format!("{}.cipher", out))?;
            file_cipher.write_all(&cipher)?;

            println!("Shared secret saved to {}.shared", out);
            println!("Ciphertext saved to {}.cipher (Send this to recipient)", out);
        }
        Commands::Decap { sec_key, cipher, out } => {
            let mut sk_buf = [0u8; pqc_kyber::KYBER_SECRETKEYBYTES];
            let mut f_sk = File::open(sec_key)?;
            f_sk.read_exact(&mut sk_buf)?;

            let mut cipher_buf = [0u8; pqc_kyber::KYBER_CIPHERTEXTBYTES];
            let mut f_c = File::open(cipher)?;
            f_c.read_exact(&mut cipher_buf)?;

            println!("Decapsulating shared secret...");
            let secret = CryptoManager::decapsulate_secret(&cipher_buf, &sk_buf)?;

            let mut file_secret = File::create(format!("{}.shared", out))?;
            file_secret.write_all(&secret)?;

            println!("Shared secret recovered and saved to {}.shared", out);
        }
        Commands::Embed { image, secret, message, out } => {
            let mut secret_buf = [0u8; pqc_kyber::KYBER_SSBYTES];
            let mut f = File::open(secret)?;
            f.read_exact(&mut secret_buf)?;

            println!("Encrypting message...");
            let (nonce, ciphertext) = CryptoManager::encrypt_aes(&secret_buf, message.as_bytes())?;

            // Format payload: [Nonce (12 bytes) | Ciphertext (N bytes)]
            let mut payload = Vec::new();
            payload.extend_from_slice(&nonce);
            payload.extend_from_slice(&ciphertext);

            println!("Embedding payload ({} bytes) into image...", payload.len());
            let img = image::open(&image)?;

            let output_img = StegoEngine::embed_message(&img, &payload)
                .map_err(|e| format!("Stego error: {}", e))?;

            output_img.save(&out)?;
            println!("Stego image saved to {:?}", out);
        }
        Commands::Extract { image, secret } => {
            let mut secret_buf = [0u8; pqc_kyber::KYBER_SSBYTES];
            let mut f = File::open(secret)?;
            f.read_exact(&mut secret_buf)?;

            println!("Extracting payload from image...");
            let img = image::open(&image)?;
            let payload = StegoEngine::extract_message(&img)
                .map_err(|e| format!("Stego error: {}", e))?;

            if payload.len() < 12 {
                return Err("Payload too short to contain nonce".into());
            }

            let (nonce, ciphertext) = payload.split_at(12);

            println!("Decrypting message...");
            let plaintext_bytes = CryptoManager::decrypt_aes(&secret_buf, nonce, ciphertext)?;
            let plaintext = String::from_utf8(plaintext_bytes)?;

            println!("Message: {}", plaintext);
        }
    }

    Ok(())
}
