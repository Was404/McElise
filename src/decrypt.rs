use crate::keygen::PrivateKey;
use crate::goppa_code::decode;
use crate::text_utils::bits_to_text;
use crate::config::Config;

pub fn decrypt(sk: &PrivateKey, ciphertext: &[u8], config: &Config) -> String {
    let n = config.n;
    let k = config.k;
    let t = config.t;
    
    let mut message_bits = Vec::new();
    
    for block in ciphertext.chunks(n) {
        if block.len() != n {
            continue;
        }
        let c: Vec<u8> = block.to_vec();
        
        let mut c_prime = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                if sk.p_inv[j][i] == 1 && c[j] == 1 {
                    c_prime[i] ^= 1;
                }
            }
        }
        
        let d = decode(&c_prime, sk.goppa_poly, &sk.support, t);
        
        let u = &d[0..k];
        
        let mut m = vec![0; k];
        for i in 0..k {
            for j in 0..k {
                if sk.s_inv[j][i] == 1 && u[j] == 1 {
                    m[i] ^= 1;
                }
            }
        }
        
        message_bits.extend(m);
    }
    for &b in &message_bits {
        assert!(b == 0 || b == 1, "message_bits contains non-binary value: {}", b);
    }
    bits_to_text(&message_bits)
}