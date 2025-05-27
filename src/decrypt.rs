use crate::keygen::PrivateKey;
use crate::text_utils::bits_to_text;

pub fn decrypt(sk: &PrivateKey, ciphertext: &[u8]) -> Vec<u8> {
    let mut msg = vec![0; sk.s.len()];
    
    // Применяем обратную перестановку
    let mut c = vec![0; ciphertext.len()];
    for i in 0..ciphertext.len() {
        for j in 0..ciphertext.len() {
            c[i] ^= sk.p_inv[i][j] * ciphertext[j];
        }
        c[i] %= 2;
    }
    
    // Упрощенное декодирование (без реального исправления ошибок)
    let decoded = &c[..sk.s.len()];
    
    // "Умножаем" на S inverse (для примера считаем S = I)
    decoded.to_vec()
    bits_to_text(&decoded_bits)
}