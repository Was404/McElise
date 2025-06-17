pub fn text_to_bits(text: &str) -> Vec<u8> {
    text.as_bytes().iter()
        .flat_map(|&byte| { // Используем pattern matching для копирования значения
            (0..8).rev().map(move |i| (byte >> i) & 1)
        })
        .collect()
}

pub fn bits_to_text(bits: &[u8]) -> String {
    let bytes: Vec<u8> = bits.chunks(8)
        .map(|chunk| {
            chunk.iter()
                .enumerate()
                .map(|(i, &bit)| bit << (7 - i % 8))
                .sum()
        })
        .collect();
    String::from_utf8_lossy(&bytes).to_string()
}