use crate::config::Config;

pub fn generate_goppa_code(config: &Config) -> (Vec<Vec<u8>>, Vec<usize>, u32) {
    let mut g = vec![vec![0; config.n]; config.k];
    for i in 0..config.k {
        g[i][i] = 1;
        for j in config.k..config.n {
            g[i][j] = ((i + j) % 2) as u8;
        }
    }
    let support = (0..config.n).collect();
    let goppa_poly = 0b10011;
    (g, support, goppa_poly)
}

pub fn decode(received: &[u8], _goppa_poly: u32, _support: &[usize], _t: usize) -> Vec<u8> {
    received.to_vec()
}