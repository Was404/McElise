use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub n: usize,   // Длина кода
    pub k: usize,   // Размерность кода
    pub t: usize,   // Количество ошибок
    pub m: usize,   // Степень расширения поля GF(2^m)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            n: 10,    // Пример для тестов
            k: 5,
            t: 2,
            m: 4,
        }
    }
}