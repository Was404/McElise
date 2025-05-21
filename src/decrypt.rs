use crate::keygen::PrivateKey; 

pub fn decrypt(sk: &PrivateKey, ciphertext: &[u8]) -> Vec<u8> {
    // Упрощенное декодирование
    let mut msg = vec![0; sk.s.len()];
    for (i, row) in sk.s.iter().enumerate() {
        msg[i] = row.iter().zip(ciphertext).map(|(&s, &c)| s * c).sum::<u8>() % 2;
    }
    msg
}