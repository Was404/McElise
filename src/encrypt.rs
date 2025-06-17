//use crate::keygen::PublicKey; // Исправляем импорт
use rand::{thread_rng, seq::SliceRandom};
use crate::text_utils::text_to_bits;

pub fn encrypt(pk: &PublicKey, message: &str, t: usize) -> Vec<u8> {
    let msg_bytes = text_to_bits(message);
    let k = pk.matrix.len();
    let n = pk.matrix[0].len();
    
    let mut ciphertext = Vec::new();
    
    // Обрабатываем сообщение блоками по k бит
    for chunk in msg_bytes.chunks(k) {
        let mut block = vec![0; n];
        let mut padded_chunk = chunk.to_vec();
        padded_chunk.resize(k, 0); // Дополняем нулями
        
        for j in 0..n {
            for i in 0..k {
                block[j] ^= padded_chunk[i] & pk.matrix[i][j];
            }
        }
        
        // Добавляем ошибки
        if t > 0 {
            let mut rng = thread_rng();
            let mut indices: Vec<usize> = (0..n).collect();
            indices.shuffle(&mut rng);
            for &i in indices.iter().take(t) {
                block[i] ^= 1;
            }
        }
        
        ciphertext.extend(block);
    }
    
    ciphertext
}