// extern crate chrono;
use clap::Parser;
// use wallit::{self, Args, Commands};
use base64::engine::{general_purpose, Engine as _};
use wallit::table_ops::history::models::NewHistory;
use wallit::*;
// mod table_ops;
use chrono::offset::{Local, Utc};
use chrono::DateTime;
use diesel::dsl::{exists, select};
use std::time::SystemTime;

fn main() {
    // println!("Hello, world!");
    // let last_modified = SystemTime::now();
    // let last_modified: DateTime<Utc> = last_modified.into();
    // // last_modified.into::<DateTime<Utc>>().format("%Y-%m-%d %T"))
    // println!("{}", last_modified.format("%Y-%m-%d %T"));//%H:%M:%S
    // let last_modified: DateTime<Local> = last_modified.into();
    // println!("{}", last_modified.format("%Y-%m-%d %T"));//%H:%M:%S

    let args = Args::parse();
    let _debug = args.debug;
    let _pool = get_connection_pool();
    let mut conn = _pool.get().unwrap();
    // println!("{:?}", args);
    match &args.command {
        Some(Commands::Add(
            // on command line the fields need to be single quoted if it has special characters such as space, &, ! etc...
            // table,
            SharedArgs {
                company,
                login,
                password,
                url,
                email,
                description,
                // remaining,
            },
        )) => {
            // println!("left over args {:?}", remaining);
            // println!("adding login {} {}",  company, login.is_some());
            // let value = if let Some(value) = login {
            //     value
            // } else {
            //     "https://test.url"
            // };

            use self::schema::logins::dsl::{company_id, logins};
            use diesel::prelude::*;
            // use table_ops::logins::models::Login;

            let company_exists =
                select(exists(logins.filter(company_id.eq(company)))).get_result::<bool>(&mut conn);
            // println!("dooes it exist {}", company_exists.unwrap());
            if company_exists.unwrap() {
                println!("company {} exists already", company);
                println!(
                    "please use: 'wallit show -t logins -c {}' to display login details",
                    company
                );
                println!("use 'wallit update -c {}' to update individual or several fields; see wallit update -h for help", company);
                return;
            }

            // combine below three statments into one didn't work
            let last_modified = SystemTime::now();
            let last_modified: DateTime<Local> = last_modified.into();
            let last_modified = last_modified.format("%Y-%m-%d %T"); //%H:%M:%S
            use table_ops::logins::models::NewLogin;
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
        Some(Commands::Update(SharedArgs {
            company,
            login,
            password,
            url,
            email,
            description,
            // remaining,
        })) => {
            use self::schema::logins::dsl::{company_id, logins};
            use diesel::prelude::*;
            let company_exists =
                select(exists(logins.filter(company_id.eq(company)))).get_result::<bool>(&mut conn);
            // println!("dooes it exist {}", company_exists.unwrap());
            if !company_exists.unwrap() {
                println!("company {} doesn't exist", company);
                return;
            }

            use table_ops::history::models::History;

            use self::schema::history::dsl::{company_id as cid, history, history_id};
            let result: Vec<History> = history
                .filter(cid.eq(company))
                .order(history_id.desc())
                .load::<History>(&mut conn)
                .expect("can't get history_id from history table");
            let hist_id = if result.first().is_none() {
                1
            } else {
                result.first().unwrap().history_id + 1
            };

            use table_ops::logins::models::Login;
            let comp: Vec<Login> = logins
                .filter(company_id.eq(company))
                .load::<Login>(&mut conn)
                .expect("wallit show -t logins -c [company]");
            let last_modified = SystemTime::now();
            let last_modified: DateTime<Local> = last_modified.into();
            let last_modified = last_modified.format("%Y-%m-%d %T"); //%H:%M:%S
                                                                     // use table_ops::history::models::NewHistory;
            let r = comp.first().unwrap();
            let new_history = &NewHistory {
                company_id: &r.company_id,
                login: &r.login,
                password: &r.password,
                email: &r.email,
                description: &r.description,
                url: &r.url,
                loginLastModified: &r.lastModified,
                lastModified: &last_modified.to_string(),
                mode: "UPDATE",
                history_id: hist_id,
            };

            use self::schema::logins::dsl::{
                description as d, email as e, lastModified as lm, login as l, password as p,
                url as u,
            };
            // let mut v = vec![];
            // _ = v.len();
            // if login.is_some() {v.push(l.eq(login.unwrap()));};
            // if password.is_some() {v.push(p.eq(password.unwrap()));};
            // if url.is_some() {v.push(url);};
            // if description.is_some() {v.push(description);};
            // if email.is_some() {v.push(email);};

            // let x = diesel::update(logins.filter(company_id.eq(company)))
            //      .set((company_id.eq(company),l.eq("")));

            // let new_login = &NewLogin {
            //     company_id: company,
            //     login: &login.clone().unwrap_or(None),
            //     password: &password.clone().unwrap_or("".to_string()),
            //     email: &email.clone().unwrap_or("".to_string()),
            //     description: &description.clone().unwrap_or("".to_string()),
            //     url: &url.clone().unwrap_or("".to_string()),
            //     lastModified: &last_modified.to_string(),
            // };
            if login.is_some() {
                let x = diesel::update(logins.filter(company_id.eq(company)))
                    .set((company_id.eq(company), l.eq(login.clone().unwrap())))
                    .get_result::<Login>(&mut conn); // doesn't work without https://stackoverflow.com/questions/74578751/diesel-get-results-gives-a-trait-bound-error
                println!("login updated {}", x.unwrap().company_id); // todo: these println strings should be concatenated
            }

            if password.is_some() {
                let x = diesel::update(logins.filter(company_id.eq(company)))
                    .set((company_id.eq(company), p.eq(password.clone().unwrap())))
                    .get_result::<Login>(&mut conn);
                println!("password updated {}", x.unwrap().company_id);
            }

            if email.is_some() {
                let x = diesel::update(logins.filter(company_id.eq(company)))
                    .set((company_id.eq(company), e.eq(email.clone().unwrap())))
                    .get_result::<Login>(&mut conn);
                println!("email updated {}", x.unwrap().company_id);
            }
            if url.is_some() {
                let x = diesel::update(logins.filter(company_id.eq(company)))
                    .set((company_id.eq(company), u.eq(url.clone().unwrap())))
                    .get_result::<Login>(&mut conn);
                println!("url updated {}", x.unwrap().company_id);
            }
            if description.is_some() {
                let x = diesel::update(logins.filter(company_id.eq(company)))
                    .set((company_id.eq(company), d.eq(description.clone().unwrap())))
                    .get_result::<Login>(&mut conn);
                println!("description updated {}", x.unwrap().company_id);
            }

            let last_modified = SystemTime::now();
            let last_modified: DateTime<Local> = last_modified.into();
            let last_modified = last_modified.format("%Y-%m-%d %T"); //%H:%M:%S

            let x = diesel::update(logins.filter(company_id.eq(company)))
                .set((company_id.eq(company), lm.eq(last_modified.to_string())))
                .get_result::<Login>(&mut conn);
            println!("lastModified updated {}", x.unwrap().company_id);

            use table_ops::history::actions::add_history;
            add_history(&mut conn, new_history);
        }
        Some(Commands::Show {
            table,
            company,
            limit,
        }) => match table.as_str() {
            // "companies" => {
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
            // }
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
                    if result.is_empty() {
                        println!("no such company found");
                        return;
                    }
                    for r in result {
                        println!(
                            "{} {} {} {} {} {}",
                            r.company_id, r.login, r.password, r.email, r.url, r.description
                        )
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
            }
            "history" => {
                // this is essentially the same as logins; how to avoid duplication
                use self::schema::history::dsl::*;
                use diesel::prelude::*;
                use table_ops::history::models::History;

                if company.is_some() {
                    // todo: sort by history_id descending
                    let company = company.clone().unwrap();
                    let result: Vec<History> = history
                        .filter(company_id.eq(company))
                        .order(history_id.desc())
                        .load::<History>(&mut conn)
                        .expect("wallit show -t history -c [company]");
                    if result.is_empty() {
                        println!("no history record found");
                        return;
                    }
                    for r in result {
                        println!(
                            "{} {} {} {} {} {}",
                            r.company_id, r.login, r.password, r.email, r.url, r.description
                        )
                    }
                } else if let Some(limit) = limit {
                    // FIXME: sort by lastModified descending
                    let result: Vec<History> = history
                        .limit(*limit)
                        .order(lastModified.desc())
                        .load::<History>(&mut conn)
                        .expect("wallit show -t history -l [limit]");
                    for r in result {
                        println!("{}", r.company_id)
                    }
                } else {
                    println!("correct use: wallit show -t history -c [company] or wallit show -t history -l [limit]")
                }
            }
            _ => (),
        },

        Some(Commands::Delete { company }) => {
            use self::schema::logins::dsl::{company_id, logins};
            use diesel::prelude::*;
            // use table_ops::logins::models::Login;

            let company_exists =
                select(exists(logins.filter(company_id.eq(company)))).get_result::<bool>(&mut conn);
            if !company_exists.unwrap() {
                println!(
                    "can not delete nonexistent company {} in logins table",
                    company
                );
                return;
            }

            // let company = company.clone().unwrap();
            use table_ops::history::models::History;

            use self::schema::history::dsl::{company_id as cid, history};
            let result: Vec<History> = history
                .filter(cid.eq(company))
                .load::<History>(&mut conn)
                .expect("can't load history table");
            let mut maximum = 0; //todo: can we use usize - need to update model
            for r in result {
                // probably should sort by history_id descending, take the first record
                maximum = std::cmp::max(r.history_id, maximum);
            }

            use table_ops::logins::models::Login;
            let result: Vec<Login> = logins
                .filter(company_id.eq(company))
                .load::<Login>(&mut conn)
                .expect("can't load logins table");
            let last_modified = SystemTime::now();
            let last_modified: DateTime<Local> = last_modified.into();
            let last_modified = last_modified.format("%Y-%m-%d %T"); //%H:%M:%S
                                                                     // use table_ops::history::models::NewHistory;
            let r = result.first().unwrap();
            // let mut new_history: &NewHistory;
            // for r in result {
            let new_history = &NewHistory {
                company_id: &r.company_id,
                login: &r.login,
                password: &r.password,
                email: &r.email,
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

    // use table_ops::logins::models::NewLogin;

    // let p = "zk$j7gq-0h";
    // // let enc = cm.encrypt(p).unwrap();
    // // let orig = &general_purpose::STANDARD_NO_PAD.encode(enc);
    // let new_login = &NewLogin {
    //     company_id: "standrpoor",
    //     login: "abc452",
    //     password: p,
    //     email: "t9frq@awsai.io",
    //     description: "",
    //     url: "",
    //     lastModified: "",
    // };
    // use table_ops::logins::actions::reveal;
    // // add_login(&mut conn, new_login);
    // let res = reveal(&mut conn, "etrade"); // using citibank (created before schema changes) Error loading login: DeserializationError(UnexpectedNullError)', src/table_ops/logins/actions.rs:21:10
    // for l in res {
    //     println!("username {}", l.login);
    //     // let outcome = cm.decrypt(decoded).unwrap();
    //     // assert_eq!(b"zk$j7gq-0h", out);
    //     // println!("after decoding: {}", std::str::from_utf8(&outcome).unwrap());

    //     println!("username {}", l.email);
    // }
}
