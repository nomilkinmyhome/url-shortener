use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn shorten_url() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}
