use clap::Parser;
use rustypassword::cli::Args;
use rustypassword::password;

fn main() {
    let args = Args::parse();
    let mut rng = rand::rng();
    let password = password::generate(&mut rng, args.length);

    println!("Password: {}", password);
}
