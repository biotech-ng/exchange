use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub trait RandomSample {
    fn new_random(len: usize) -> Self;
}

impl RandomSample for String {
    fn new_random(len: usize) -> Self {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}
