use wallit::Args;
use clap::Parser;
fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}", args.name);
    }

}
