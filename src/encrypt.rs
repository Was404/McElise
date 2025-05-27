use crate::keygen::PublicKey; // Исправляем импорт
use rand::{thread_rng, seq::SliceRandom};
use crate::text_utils::text_to_bits;

pub fn encrypt(pk: &PublicKey, message: &str, t: usize) -> Vec<u8> {
    let msg_bits = text_to_bits(message);
    let mut c = vec![0; pk.matrix[0].len()];
    for (i, col) in pk.matrix.iter().enumerate() {
        c[i] = message.iter().zip(col).map(|(&m, c)| m * c).sum::<u8>() % 2;
    }
    
    let mut rng = thread_rng();
    let mut error = vec![0; c.len()];
    let mut indices: Vec<usize> = (0..c.len()).collect();
    indices.shuffle(&mut rng);
    for &i in indices.iter().take(t) {
        error[i] ^= 1;
    }
    
    c.iter().zip(error).map(|(&a, b)| (a + b) % 2).collect()
}