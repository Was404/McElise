pub fn text_to_bits(text: &str) -> Vec<u8> {
    text.as_bytes().iter()
        .flat_map(|&byte| { // Используем pattern matching для копирования значения
            (0..8).rev().map(move |i| (byte >> i) & 1)
        })
        .collect()
}

pub fn bits_to_text(bits: &[u8]) -> String {
    let mut bytes = Vec::new();
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit > 1 {
                panic!("Bit value > 1: {}", bit);
            }
            byte |= bit << (7 - i);
        }
        bytes.push(byte);
    }
    String::from_utf8_lossy(&bytes).to_string()
}