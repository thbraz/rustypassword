use rand::distr::{Alphanumeric, SampleString};

pub fn gen_password(length: u8) -> String {
    let mut rng = rand::rng();
    Alphanumeric.sample_string(&mut rng, length as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_length() {
        let length: u8 = 8;
        let password = gen_password(length);
        assert_eq!(password.len(), length as usize);
    }

    fn length_zero() {
        let length: u8 = 0;
        let password = gen_password(length);
        assert_eq!(password.len(), length as usize);
    }
}
