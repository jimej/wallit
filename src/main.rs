use clap::Parser;
// use wallit::{self, Args, Commands};
use base64::engine::{general_purpose, Engine as _};
use wallit::*;
// mod table_ops;
fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let _debug = args.debug;
    let _pool = get_connection_pool();
    let mut conn = _pool.get().unwrap();
    match &args.command {
        Some(Commands::Add {
            tbl,
            company,
            value,
        }) => {
            println!("adding company {} {} {}", tbl, company, value.is_some());
            let value = if let Some(value) = value {
                value
            } else {
                "https://test.url"
            };
            match tbl.as_str() {
                "companies" => {
                    use table_ops::companies::actions::add_company;
                    let _res = add_company(&mut conn, company, value);
                    println!("number of companies added: {}, name: {}", _res, company);
                }
                "logins" => (),
                _ => println!("can't add to such table"),
            }
        }
        Some(Commands::Show { tbl, all, company }) => match tbl.as_str() {
            "companies" => {
                use self::schema::companies::dsl::*;
                use diesel::prelude::*;
                use table_ops::companies::models::Company;
                if *all {
                    let results: Vec<Company> = companies
                        .limit(10)
                        .load::<Company>(&mut conn)
                        .expect("failed to load companies");
                    for r in results {
                        println!("{} {} ", r.company_id, r.url);
                    }
                } else if company.is_some() {
                    let company = company.clone().unwrap();
                    let results: Vec<Company> = companies
                        .filter(company_id.eq(company))
                        .load::<Company>(&mut conn)
                        .expect("failed to load companies");
                    for r in results {
                        println!("{} {} ", r.company_id, r.url);
                    }
                } else {
                    println!("wallit show -t [table] [-a] [-c company_id]");
                }
            }
            "logins" => (),
            "login_history" => (),
            _ => (),
        },
        _ => println!("not allowed subcommand"),
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

    use table_ops::logins::models::NewLogin;

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
    use table_ops::logins::actions::{add_login, reveal};
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
