use rand::distr::{Alphanumeric, SampleString};
use rand::rngs::ThreadRng;

pub fn generate(rng: &mut ThreadRng, length: u8) -> String {
    Alphanumeric.sample_string(rng, length as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_length() {
        let length: u8 = 8;
        let mut rng = rand::rng();
        let password = generate(&mut rng, length);
        assert_eq!(password.len(), length as usize);
    }

    #[test]
    fn length_zero() {
        let length: u8 = 0;
        let mut rng = rand::rng();
        let password = generate(&mut rng, length);
        assert_eq!(password.len(), length as usize);
    }
}
