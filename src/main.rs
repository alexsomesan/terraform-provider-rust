use std::env;

pub mod server;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Terraform Provider Rust: ");
    for (n, v) in env::vars() {
        print!("{}: {}\t", n, v);
    }
    println!("Over and out");
    Ok(())
}
