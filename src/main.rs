use clap::Parser;
use wallit::{Args, Commands};
fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let _debug = args.debug;

    match &args.command {
        Some(Commands::Add { key, value }) if _debug => {
            println!("adding secrets {} {}", key, value)
        }
        Some(_) if !_debug => println!("with actions but no debug"),
        _ => println!("not adding secrets"),
    }
}
