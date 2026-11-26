use clap::Parser;
use rustypassword::cli::Args;

fn main() {
    let args = Args::parse();
    println!("Value: {}", args.length);
}
