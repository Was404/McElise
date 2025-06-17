use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn multiply_matrices(a: &[Vec<u8>], b: &[Vec<u8>], rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut result = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            result[i][j] = a[i].iter().zip(b.iter()).map(|(x, row)| x & row[j]).sum::<u8>() % 2;
        }
    }
    result
}

pub fn generate_invertible_matrix(size: usize) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![0; size]; size];
    for i in 0..size {
        matrix[i][i] = 1;
    }
    matrix
}

pub fn generate_permutation_matrix(size: usize) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut rng = thread_rng();
    let mut perm: Vec<usize> = (0..size).collect();
    perm.shuffle(&mut rng);
    
    let mut matrix = vec![vec![0; size]; size];
    let mut inv_matrix = vec![vec![0; size]; size];
    for (i, &j) in perm.iter().enumerate() {
        matrix[i][j] = 1;
        inv_matrix[j][i] = 1;
    }
    (matrix, inv_matrix)
}

pub fn inverse_matrix(matrix: &[Vec<u8>]) -> Option<Vec<Vec<u8>>> {
    Some(matrix.to_vec())
}