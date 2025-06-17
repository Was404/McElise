mod config;
mod keygen;
mod matrix_utils;
mod goppa_code;
mod encrypt;
mod decrypt;
mod stern_attack;
mod text_utils;
mod bit_utils; // new

use crate::bit_utils::{pack_bits, unpack_bits}; // используй где нужно
use std::fs;
use bincode;
use clap::{Parser, Subcommand};
use crate::config::Config;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    GenerateKeys,
    Encrypt { message: String },
    Decrypt { ciphertext_file: String },
    Attack,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = Config::default();
    
    match cli.command {
        Commands::GenerateKeys => {
            keygen::generate_and_save_keys(&config)?;
            println!("Keys generated and saved to public_key.bin and private_key.bin.");
        }
        Commands::Encrypt { message } => {
            let pk_bytes = fs::read("public_key.bin")?;
            let pk: keygen::PublicKey = bincode::deserialize(&pk_bytes)?;
            let ciphertext = encrypt::encrypt(&pk, &message, config.t);
            let packed_ct = pack_bits(&ciphertext);
            let mut output = Vec::new();
            let msg_len = message.len() as u32;
            output.extend(&msg_len.to_le_bytes());
            output.extend(&packed_ct);
            fs::write("ciphertext.bin", &output)?;
            println!("Ciphertext saved to ciphertext.bin");
            println!("Hex: {}", hex::encode(&packed_ct));
        }
        Commands::Decrypt { ciphertext_file } => {
            let sk_bytes = fs::read("private_key.bin")?;
            let sk: keygen::PrivateKey = bincode::deserialize(&sk_bytes)?;
            let packed_ct = fs::read(&ciphertext_file)?;
            let msg_len_bytes = &packed_ct[..4];
            let _msg_len = u32::from_le_bytes([msg_len_bytes[0], msg_len_bytes[1], msg_len_bytes[2], msg_len_bytes[3]]) as usize;
            let packed_ct_data = &packed_ct[4..];
            let ct = unpack_bits(packed_ct_data, None);
            let msg = decrypt::decrypt(&sk, &ct, &config);
            fs::write("decrypted.txt", msg.as_bytes())?;
            println!("Decrypted text saved to decrypted.txt: {}", msg);
        }
        Commands::Attack => {
            let pk_bytes = fs::read("public_key.bin")?;
            let pk: keygen::PublicKey = bincode::deserialize(&pk_bytes)?;
            let config = Config::default();
            if let Some(error_vectors) = stern_attack::stern_attack(&pk, config.n, config.k, config.t) {
                println!("Found potential error vectors: {:?}", error_vectors);
            } else {
                println!("No error vector found.");
            }
            if let Some(msg) = stern_attack::stern_attack_and_recover(
                &pk,
                config.n,
                config.k,
                config.t,
                "ciphertext.bin",
            ) {
                println!("Recovered message: {}", msg);
            } else {
                println!("No error vector found or failed to recover message.");
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_conversion() {
        let text = "Test";
        let bits = text_utils::text_to_bits(text);
        let back = text_utils::bits_to_text(&bits);
        assert_eq!(text, back);
    }
}