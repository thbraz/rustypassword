use clap::Parser;
use rustypassword::cli::Args;
use rustypassword::generator;

fn main() {
    let args = Args::parse();
    let password = generator::gen_password(args.length);

    println!("Password: {}", password);
}
