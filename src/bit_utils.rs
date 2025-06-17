pub fn pack_bits(bits: &[u8]) -> Vec<u8> {
    let mut packed = Vec::new();
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << i;
        }
        packed.push(byte);
    }
    packed
}

pub fn unpack_bits(packed: &[u8], bit_len: Option<usize>) -> Vec<u8> {
    let bits: Vec<u8> = packed.iter().flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1)).collect();
    if let Some(len) = bit_len {
        bits.into_iter().take(len).collect()
    } else {
        bits
    }
}