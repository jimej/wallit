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
    println!("{:?}", args);
    match &args.command {
        Some(Commands::Add {
            tbl,
            company,
            value,
            remaining,
        }) => {
            println!("left over args {:?}", remaining);
            println!("adding company {} {} {}", tbl, company, value.is_some());
            let value = if let Some(value) = value {
                value
            } else {
                "https://test.url"
            };
            match tbl.as_str() {
                "companies" => {
                    use table_ops::companies::actions::add_company;
                    let _res = add_company(&mut conn, company, value, "arbitrary");
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

    use table_ops::logins::models::NewLogin;

    let p = "zk$j7gq-0h";
    // let enc = cm.encrypt(p).unwrap();
    // let orig = &general_purpose::STANDARD_NO_PAD.encode(enc);
    let new_login = &NewLogin {
        company_id: "standrpoor",
        login: "abc452",
        password: p,
        email: "t9frq@awsai.io",
        description: "",
        url: "",
        lastModified: "",
    };
    use table_ops::logins::actions::{add_login, reveal};
    // add_login(&mut conn, new_login);
    let res = reveal(&mut conn, "etrade"); // using citibank (created before schema changes) Error loading login: DeserializationError(UnexpectedNullError)', src/table_ops/logins/actions.rs:21:10
    for l in res {
        println!("username {}", l.login);
        // let outcome = cm.decrypt(decoded).unwrap();
        // assert_eq!(b"zk$j7gq-0h", out);
        // println!("after decoding: {}", std::str::from_utf8(&outcome).unwrap());

        println!("username {}", l.email);
    }
}
