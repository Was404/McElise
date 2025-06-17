use crate::goppa_code::generate_goppa_code;
use crate::matrix_utils::{generate_invertible_matrix, generate_permutation_matrix, multiply_matrices, inverse_matrix};
use serde::{Serialize, Deserialize};
use crate::config::Config;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPair {
    pub public: PublicKey,
    pub private: PrivateKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicKey {
    pub matrix: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateKey {
    pub s: Vec<Vec<u8>>,
    pub s_inv: Vec<Vec<u8>>,
    pub p: Vec<Vec<u8>>,
    pub p_inv: Vec<Vec<u8>>,
    pub goppa_poly: u32,
    pub support: Vec<usize>,
}

pub fn generate_and_save_keys(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (public, private) = generate_keys(config);
    let key_pair = KeyPair { public, private };
    
    let public_bytes = bincode::serialize(&key_pair.public)?;
    let private_bytes = bincode::serialize(&key_pair.private)?;
    
    fs::write("public_key.bin", public_bytes)?;
    fs::write("private_key.bin", private_bytes)?;
    
    Ok(())
}

fn generate_keys(config: &Config) -> (PublicKey, PrivateKey) {
    let (g, support, goppa_poly) = generate_goppa_code(config);
    let s = generate_invertible_matrix(config.k);
    let s_inv = inverse_matrix(&s).expect("S must be invertible");
    let (p, p_inv) = generate_permutation_matrix(config.n);
    let s_g = multiply_matrices(&s, &g, config.k, config.n);
    let g_prime = multiply_matrices(&s_g, &p, config.k, config.n);
    
    (PublicKey { matrix: g_prime }, PrivateKey { s, s_inv, p, p_inv, goppa_poly, support })
}