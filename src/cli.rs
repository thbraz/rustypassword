use clap::Parser;

#[derive(Parser)]
#[command(name = "Rusty Password", version = "0.0.1", about = "Password Generator written in Rust")]
pub struct Args {
    #[arg(short, long)]
    pub length: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert();
    }

    #[test]
    fn parses_length() {
        let args = Args::try_parse_from(["rustypassword", "-l", "16"]).unwrap();
        assert_eq!(args.length, 16);

        let args = Args::try_parse_from(["rustypassword", "--length", "32"]).unwrap();
        assert_eq!(args.length, 32);
    }

    #[test]
    fn length_required() {
        assert!(Args::try_parse_from(["rustypassword"]).is_err());
    }

    #[test]
    fn rejects_out_of_range() {
        assert!(Args::try_parse_from(["rustypassword", "-l", "300"]).is_err());
    }

}
