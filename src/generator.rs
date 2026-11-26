use rand::distr::{Alphanumeric, SampleString};

pub fn gen_password(length: u8) -> String {
    let mut rng = rand::rng();
    Alphanumeric.sample_string(&mut rng, length as usize)
}
