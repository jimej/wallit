use clap::Parser;
// use wallit::{self, Args, Commands};
use base64::engine::{general_purpose, Engine as _};
use wallit::*;
mod table_ops;
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
    let input = "input hello";
    let res = cm.encrypt(input); // Ok(s), encrypted text can't conver to UTF-8 from bytes
    println!("encryption success: {}", res.is_ok());
    // println!("encrypted text: {}", std::str::from_utf8(&res).unwrap());
    if let Ok(secret) = res {
        println!("encrypted text: {:?}", secret);
        let out = cm.decrypt(secret).unwrap();
        assert_eq!(&out, input.as_bytes());
        println!("after decryption: {}", std::str::from_utf8(&out).unwrap());
    }

    let _pool = get_connection_pool();
    let mut conn = _pool.get().unwrap();
    use self::schema::companies::dsl::*;
    use diesel::prelude::*;
    let records = vec![
        (company_id.eq("citibank"), url.eq("https://citi.com")),
        (
            company_id.eq("discover"),
            url.eq("https://discoverbank.com"),
        ),
    ];

    let _res = diesel::insert_into(companies)
        .values(&records)
        .execute(&mut conn);
    match _res {
        Err(e) => println!("{e}"),
        Ok(_) => println!("great!!!!"),
    }
    use table_ops::companies::actions::add_company;
    let _res = add_company(&mut conn, "etrade", "https://www.etrade.com");
    println!("number of companies added: {}", _res);

    use self::models::NewLogin;

    let p = "zk$j7gq-0h";
    let enc = cm.encrypt(p).unwrap();
    let orig = &general_purpose::STANDARD_NO_PAD.encode(enc);
    let new_login = &NewLogin {
        company_id: "citibank",
        username: "abc452",
        password: orig,
        email: "t9frq@awsai.io",
        history_id: 1,
    };
    add_login(&mut conn, new_login);
    let res = reveal(&mut conn, "citibank");
    for l in res {
        println!("username {}", l.username);
        let decoded = general_purpose::STANDARD_NO_PAD.decode(l.password).unwrap();
        let outcome = cm.decrypt(decoded).unwrap();
        // assert_eq!(b"zk$j7gq-0h", out);
        println!("after decoding: {}", std::str::from_utf8(&outcome).unwrap());

        println!("username {}", l.email);
    }
}
