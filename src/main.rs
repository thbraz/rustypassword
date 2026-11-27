use clap::Parser;
use rustypassword::cli::Args;
use rustypassword::password;

fn main() {
    let args = Args::parse();
    let mut generator = rand::rng();
    let password = password::generate(&mut generator, args.length);

    println!("Password: {}", password);
}
