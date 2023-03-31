use rand::{distributions::Alphanumeric, Rng}; // 0.8

pub fn generate_code(length:usize) -> String {
     rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
