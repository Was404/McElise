use crate::Config;

pub fn generate_goppa_code(config: &Config) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, u32) {
    // Заглушка для примера
    let g = vec![vec![1; config.n]; config.k];
    let h = vec![vec![1; config.n]; config.n - config.k];
    (g, h, 0b10011) // Пример полинома
}