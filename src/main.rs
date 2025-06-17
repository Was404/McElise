mod config;
mod keygen;
mod matrix_utils;
mod goppa_code;
mod encrypt;
mod decrypt;
mod stern_attack;
mod text_utils;

//use serde;
use std::fs;
use bincode;
use clap::{Parser, Subcommand};
use crate::config::Config;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[test]
fn test_text_conversion() {
    let text = "Test";
    let bits = text_utils::text_to_bits(text);
    let back = text_utils::bits_to_text(&bits);
    assert_eq!(text, back);
}

#[derive(Subcommand)]
enum Commands {
    GenerateKeys,
    Encrypt { message: String },
    Decrypt { ciphertext: String },
    Attack, // Теперь это unit-вариант
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = Config::default();
    
    match cli.command {
        Commands::GenerateKeys => {
            keygen::generate_and_save_keys(&config)?;
            println!("Keys generated and saved.");
        }
        Commands::Encrypt { message } => {
            let pk_bytes = fs::read("public_key.bin")?;
            let pk: keygen::PublicKey = bincode::deserialize(&pk_bytes)?;
            let ciphertext = encrypt::encrypt(&pk, &message, config.t);
            
            // Выводим в hex для удобства
            println!("Hex: {}", hex::encode(&ciphertext));
            println!("Binary: {}", ciphertext.iter()
                .map(|b| b.to_string()).collect::<String>());
        }
        
        Commands::Decrypt { ciphertext } => {
            let sk_bytes = fs::read("private_key.bin")?;
            let sk: keygen::PrivateKey = bincode::deserialize(&sk_bytes)?;
            
            let ct = if ciphertext.starts_with("0x") {
                hex::decode(&ciphertext[2..])?
            } else {
                ciphertext.chars()
                    .map(|c| match c {
                        '0' => Ok(0),
                        '1' => Ok(1),
                        _ => Err("Invalid binary character"),
                    })
                    .collect::<Result<Vec<u8>, _>>()
                    .map_err(|e| Box::<dyn std::error::Error>::from(e))?
            };
            
            let msg = decrypt::decrypt(&sk, &ct);
            println!("Decrypted: {}", msg);
        }
        Commands::Attack => { // Правильный формат для unit-варианта
            let pk_bytes = fs::read("public_key.bin")?;
            let pk: keygen::PublicKey = bincode::deserialize(&pk_bytes)?;
            let config = Config::default();
        
            if let Some(errors) = stern_attack::stern_attack(
                &pk,
                config.n,
                config.k,
                config.t
                ) {
                println!("Found potential error vectors: {:?}", errors);
            } else {
                println!("Attack failed after max iterations");
            }
        }
    }
    
    Ok(())
}