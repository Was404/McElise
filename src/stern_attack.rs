use rand::{thread_rng, seq::SliceRandom};
use std::collections::HashMap;

/// Проверочная матрица H размера (n-k) x n
fn compute_parity_check_matrix(_g: &[Vec<u8>], n: usize, k: usize) -> Vec<Vec<u8>> {
    // Для теста: H = [I | P], где I — (n-k)x(n-k), P — (n-k)xk
    // Реально H строится как ортогональное дополнение к G
    let mut h = vec![vec![0u8; n]; n - k];
    for i in 0..(n - k) {
        h[i][k + i] = 1;
    }
    h
}

/// Проверка, что подматрица columns из H — единичная
fn is_identity_submatrix(h: &[Vec<u8>], columns: &[usize]) -> bool {
    let size = columns.len();
    for i in 0..size {
        for j in 0..size {
            if h[i][columns[j]] != if i == j { 1 } else { 0 } {
                return false;
            }
        }
    }
    true
}

/// Возвращает все подмножества заданного размера
fn all_subsets(set: &[usize], p: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let n = set.len();
    if p == 0 || p > n {
        return result;
    }
    let mut indices = (0..p).collect::<Vec<_>>();
    loop {
        result.push(indices.iter().map(|&i| set[i]).collect());
        // Следующая комбинация
        let mut i = p;
        while i > 0 {
            i -= 1;
            if indices[i] != i + n - p {
                break;
            }
        }
        if indices[0] == n - p {
            break;
        }
        indices[i] += 1;
        for j in i+1..p {
            indices[j] = indices[j-1] + 1;
        }
    }
    result
}

/// Основная функция Stern-атаки
pub fn stern_attack(pk: &super::keygen::PublicKey, n: usize, k: usize, t: usize) -> Option<Vec<u8>> {
    let h = compute_parity_check_matrix(&pk.matrix, n, k);
    let l = (n - k) / 2; // Размер группы Z
    let p = t / 2;       // Размер подмножеств A и B

    let mut rng = thread_rng();
    let max_iter = 1000;

    for _ in 0..max_iter {
        // 1. Случайное разбиение столбцов на Z, X, Y
        let mut indices: Vec<usize> = (0..n).collect();
        indices.shuffle(&mut rng);
        let z = indices[..l].to_vec();
        let rest = &indices[l..];
        let x = rest[..(rest.len()/2)].to_vec();
        let y = rest[(rest.len()/2)..].to_vec();

        // 2. Проверяем, что H[:,Z] — единичная
        if !is_identity_submatrix(&h, &z) { continue; }

        // 3. Перебираем все подмножества размера p в X и Y
        let a_subsets = all_subsets(&x, p);
        let b_subsets = all_subsets(&y, p);

        // 4. Для каждого A и B вычисляем π(A), π(B)
        let mut hash_map: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
        for a in &a_subsets {
            let mut pi_a = vec![0u8; n - k];
            for &col in a {
                for i in 0..(n - k) {
                    pi_a[i] ^= h[i][col];
                }
            }
            hash_map.insert(pi_a, a.clone());
        }
        for b in &b_subsets {
            let mut pi_b = vec![0u8; n - k];
            for &col in b {
                for i in 0..(n - k) {
                    pi_b[i] ^= h[i][col];
                }
            }
            if let Some(a) = hash_map.get(&pi_b) {
                // 5. Кандидат на решение: e' = A ∪ B
                let mut e = vec![0u8; n];
                for &col in a.iter().chain(b.iter()) {
                    e[col] = 1;
                }
                // Добавляем столбцы из Z с ненулевыми компонентами (в данном тестовом варианте — пропускаем)
                // 6. Проверяем вес
                if e.iter().filter(|&&x| x == 1).count() == t {
                    return Some(e);
                }
            }
        }
    }
    None
}