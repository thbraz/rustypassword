use rand::distr::{Alphanumeric, SampleString};

pub fn generate(length: u8) -> String {
    let mut rng = rand::rng();
    Alphanumeric.sample_string(&mut rng, length as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_length() {
        let length: u8 = 8;
        let password = generate(length);
        assert_eq!(password.len(), length as usize);
    }

    #[test]
    fn length_zero() {
        let length: u8 = 0;
        let password = generate(length);
        assert_eq!(password.len(), length as usize);
    }
}
