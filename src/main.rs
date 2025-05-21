mod config;
mod keygen;
mod matrix_utils;
mod goppa_code;
mod encrypt;
mod decrypt;
mod stern_attack;

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

#[derive(Subcommand)]
enum Commands {
    GenerateKeys,
    Encrypt { message: String },
    Decrypt { ciphertext: String },
    Attack { ciphertext: String },
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
            let msg_bin = message.bytes().map(|b| b % 2).collect::<Vec<u8>>();
            let ciphertext = encrypt::encrypt(&pk, &msg_bin, config.t);
            println!("Ciphertext: {:?}", ciphertext);
        }
        Commands::Decrypt { ciphertext } => {
            let sk_bytes = fs::read("private_key.bin")?;
            let sk: keygen::PrivateKey = bincode::deserialize(&sk_bytes)?;
            let ct = ciphertext.split(',')
                .map(|s| s.trim().parse::<u8>().unwrap() % 2)
                .collect::<Vec<_>>();
            let msg = decrypt::decrypt(&sk, &ct);
            println!("Decrypted: {:?}", msg);
        }
        Commands::Attack { ciphertext: _ } => { // Игнорируем неиспользуемый параметр
            println!("Attack not implemented yet");
        }
    }
    
    Ok(())
}