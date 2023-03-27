use clap::Parser;
use wallit::{self, Args, Commands};
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

    let cm = wallit::CipherMaterial::default();
    let res = cm.encrypt("input"); // Ok(s), can't conver to UTF-8
    println!("encryption success: {}", res.is_ok());
    // println!("encrypted text: {}", std::str::from_utf8(&res).unwrap());
    if let Ok(secret) = res {
        println!("encrypted text: {:?}", secret);
        let out = cm.decrypt(secret).unwrap();
        assert_eq!(&out, b"input");
        println!("after decryption: {}", std::str::from_utf8(&out).unwrap());
    }
}
