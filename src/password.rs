use rand::distr::{Alphanumeric, SampleString};
use rand::rngs::ThreadRng;

pub fn generate(rng: &mut ThreadRng, length: usize) -> String {
    Alphanumeric.sample_string(rng, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_length() {
        let length: usize = 8;
        let mut rng = rand::rng();
        let password = generate(&mut rng, length);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn length_zero() {
        let length: usize = 0;
        let mut rng = rand::rng();
        let password = generate(&mut rng, length);
        assert_eq!(password.len(), length);
    }
}
