use std::time::{SystemTime, UNIX_EPOCH};

use rand::{distributions::Alphanumeric, Rng};

pub fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time is before UNIX_EPOCH")
        .as_secs()
}

pub fn rand_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
