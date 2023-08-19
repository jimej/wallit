// extern crate chrono;
use clap::Parser;
// use wallit::{self, Args, Commands};
use base64::engine::{general_purpose, Engine as _};
use wallit::*;
use wallit::table_ops::history::models::NewHistory;
// mod table_ops;
use std::time::SystemTime;
use chrono::offset::{Utc, Local};
use chrono::DateTime;
use diesel::dsl::{exists, select};

fn main() {
    println!("Hello, world!");
    let last_modified = SystemTime::now();
    let last_modified: DateTime<Utc> = last_modified.into();
    // last_modified.into::<DateTime<Utc>>().format("%Y-%m-%d %T"))
    println!("{}", last_modified.format("%Y-%m-%d %T"));//%H:%M:%S
    let last_modified: DateTime<Local> = last_modified.into();
    println!("{}", last_modified.format("%Y-%m-%d %T"));//%H:%M:%S

    let args = Args::parse();
    let _debug = args.debug;
    let _pool = get_connection_pool();
    let mut conn = _pool.get().unwrap();
    println!("{:?}", args);
    match &args.command {
        Some(Commands::Add { // on command line the fields need to be single quoted if it has special characters such as space, &, ! etc...
            // table,
            company,
            login,
            password,
            url,
            email,
            description,
            // remaining,
        }) => {
            // println!("left over args {:?}", remaining);
            println!("adding login {} {}",  company, login.is_some());
            let value = if let Some(value) = login {
                value
            } else {
                "https://test.url"
            };

            use self::schema::logins::dsl::{company_id, logins};
            use diesel::prelude::*;
                // use table_ops::logins::models::Login;

            let company_exists = select(exists(logins
                .filter(company_id.eq(company))))
                .get_result::<bool>(&mut conn);
            // println!("dooes it exist {}", company_exists.unwrap());
            if company_exists.unwrap() {
                  println!("company {} exists already", company);
                  println!("please use: wallit show -t logins -c {} to display login details", company);
                  println!("use wallit update -c {} to update individual or several fields; see wallit update -h for help", company);
                  return;
            }
            
            // combine below three statments into one didn't work
            let last_modified = SystemTime::now();
            let last_modified: DateTime<Local> = last_modified.into();
            let last_modified = last_modified.format("%Y-%m-%d %T");//%H:%M:%S
            let new_login = &NewLogin {
                company_id: company,
                login: &login.clone().unwrap_or("".to_string()),
                password: &password.clone().unwrap_or("".to_string()),
                email: &email.clone().unwrap_or("".to_string()),
                description: &description.clone().unwrap_or("".to_string()),
                url: &url.clone().unwrap_or("".to_string()),
                lastModified: &last_modified.to_string(),
            };

            use table_ops::logins::actions::add_login;
            add_login(&mut conn, new_login);
            println!("added login for company {}", company)

            // match table.as_str() {
            //     "companies" => {
            //         use table_ops::companies::actions::add_company;
            //         let _res = add_company(&mut conn, company, value, "arbitrary");
            //         println!("number of companies added: {}, name: {}", _res, company);
            //     }
            //     "logins" => (),
            //     _ => println!("can't add to such table"),
            // }
        }
        Some(Commands::Show { table, company, limit }) => match table.as_str() {
            "companies" => {
                // use self::schema::companies::dsl::*;
                // use diesel::prelude::*;
                // use table_ops::companies::models::Company;
                // if *all {
                //     let results: Vec<Company> = companies
                //         .limit(10)
                //         .load::<Company>(&mut conn)
                //         .expect("failed to load companies");
                //     for r in results {
                //         println!("{} {} ", r.company_id, r.url);
                //     }
                // } else if company.is_some() {
                //     let company = company.clone().unwrap();
                //     let results: Vec<Company> = companies
                //         .filter(company_id.eq(company))
                //         .load::<Company>(&mut conn)
                //         .expect("failed to load companies");
                //     for r in results {
                //         println!("{} {} ", r.company_id, r.url);
                //     }
                // } else {
                //     println!("wallit show -t [table] [-a] [-c company_id]");
                // }
            }
            "logins" => {
                use self::schema::logins::dsl::*;
                use diesel::prelude::*;
                use table_ops::logins::models::Login;

                
                if company.is_some() {
                    let company = company.clone().unwrap();
                    let result: Vec<Login> = logins
                        .filter(company_id.eq(company))
                        .load::<Login>(&mut conn)
                        .expect("wallit show -t logins -c [company]");
                    for r in result {
                        println!("{} {} {} {} {} {}", r.company_id, r.login, r.password, r.email, r.url, r.description)
                    }
                } else if let Some(limit) = limit {
                    let result: Vec<Login> = logins
                        .limit(*limit)
                        .load::<Login>(&mut conn)
                        .expect("wallit show -t logins -l [limit]");
                    for r in result {
                        println!("{}", r.company_id)
                    }
                } else {
                    println!("correct use: wallit show -t logins -c [company] or wallit show -t logins -l [limit]")
                }

            },
            "history" => (), // this is essentially the same as logins; how to avoid duplication
            _ => (),
        },

        Some(Commands::Delete{company}) => {
            use self::schema::logins::dsl::{company_id, logins};
            use diesel::prelude::*;
                // use table_ops::logins::models::Login;

            let company_exists = select(exists(logins
                .filter(company_id.eq(company))))
                .get_result::<bool>(&mut conn);
            if !company_exists.unwrap() {
                println!("can not delete nonexistent company {} in logins table", company);
                return
            }
            
            

            // let company = company.clone().unwrap();
            use table_ops::history::models::History;

            use self::schema::history::dsl::{company_id as cid, history};
            let result: Vec<History> = history
                        .filter(cid.eq(company))
                        .load::<History>(&mut conn)
                        .expect("wallit show -t logins -c [company]");
            let mut maximum = 0;
            for r in result { // probably should sort by history_id descending, take the first record
                maximum =std::cmp::max(r.history_id, maximum);
            }

            use table_ops::logins::models::Login;
            let result: Vec<Login> = logins
                        .filter(company_id.eq(company))
                        .load::<Login>(&mut conn)
                        .expect("wallit show -t logins -c [company]");
            let last_modified = SystemTime::now();
            let last_modified: DateTime<Local> = last_modified.into();
            let last_modified = last_modified.format("%Y-%m-%d %T");//%H:%M:%S
            // use table_ops::history::models::NewHistory;
            let r = result.first().unwrap();
            // let mut new_history: &NewHistory;
            // for r in result {
            let    new_history = &NewHistory{
                company_id: &r.company_id,
                login: &r.login,
                password: &r.password,
                // email: &r.email,
                description: &r.description,
                url: &r.url,
                loginLastModified: &r.lastModified,
                lastModified: &last_modified.to_string(),
                mode: "DELETE",
                history_id: maximum + 1,
                 };
            // }
        
            use table_ops::history::actions::add_history;
            add_history(&mut conn, new_history);
            //delete from logins and insert into history should be atomic; or insert first before deletion
            _ = diesel::delete(logins.filter(company_id.eq(company))).execute(&mut conn);
            
        }

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
    use table_ops::logins::actions::{/*add_login,*/ reveal};
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
