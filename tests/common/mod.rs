use rand::{distributions::Alphanumeric, Rng}; // 0.8

pub fn generate_random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect()
    
}