use clap::Parser;
use rustypassword::cli::Args;
use rustypassword::password;

fn main() {
    let args = Args::parse();
    let password = password::generate(args.length);

    println!("Password: {}", password);
}
