use crate::keygen::PrivateKey;
use crate::text_utils::bits_to_text;

pub fn decrypt(sk: &PrivateKey, ciphertext: &[u8]) -> String {
    let n = sk.p_inv.len();
    let k = sk.s.len();
    
    let mut message_bits = Vec::new();
    
    // Обрабатываем шифртекст блоками по n бит
    for block in ciphertext.chunks(n) {
        if block.len() != n {
            continue;
        }
        
        let mut c = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                c[i] ^= sk.p_inv[i][j] & block[j];
            }
        }
        
        let mut m_s = vec![0; k];
        for i in 0..k {
            for j in 0..k {
                m_s[i] ^= c[j] & sk.s[j][i];
            }
        }
        
        message_bits.extend(m_s);
    }
    
    bits_to_text(&message_bits)
}