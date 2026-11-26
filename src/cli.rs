use clap::Parser;

#[derive(Parser)]
#[command(name = "Rusty Password", version = "0.0.1", about = "Password Generator written in Rust")]
pub struct Args {
    #[arg(short, long)]
    pub length: u8,
}
