// use crate::schema::{companies, logins};
// use diesel::prelude::*;

// // #[derive(Queryable)]
// // pub struct Company {
// //     pub id: u32,
// //     pub company_id: String,
// //     pub url: String,
// // }

// // #[derive(Insertable)]
// // #[diesel(table_name = companies)]
// // pub struct NewCompany<'a> {
// //     pub company_id: &'a str,
// //     pub url: &'a str,
// // }

// #[derive(Queryable)]
// pub struct Login {
//     pub id: i32, // u32 has issues for reveal()
//     pub company_id: String,
//     pub username: String,
//     pub password: String,
//     pub email: String,
//     pub history_id: i32, // u32 has issues for reveal()
// }

// #[derive(Insertable)]
// #[diesel(table_name = logins)]
// pub struct NewLogin<'a> {
//     pub company_id: &'a str,
//     pub username: &'a str,
//     pub password: &'a str,
//     pub email: &'a str,
//     pub history_id: i32, // u32 doesnt work, i8 doesn't work either
// }
