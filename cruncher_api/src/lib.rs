// use std::error::Error;
// mod parser;
// mod ranker;

// pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

//     println!("In file {}", config.filename);
//     //parser::generate_token_counted_stock_file(config.filename).expect("Failed to generate token counted stock file.");
//     ranker::generate_ranking(String::from("RTN")).expect("err");
//     Ok(())
// }

// pub struct Config {
//     filename: String
// }

// impl Config {
//     pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
//        args.next();

//        let filename = match args.next() {
//            Some(arg) => arg,
//            None => return Err("Didn't get a file name."),
//        };

//        Ok(Config { filename })
//     }
// }
#![feature(const_fn)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate proc_macro;
extern crate bigdecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;


pub mod logger;
pub mod models;
pub mod parser;
pub mod ranker;
pub mod repos;
pub mod services;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

//use self::models::{InsertableStock, Stock};
//pub fn create_stock<'a>(
//    conn: &PgConnection,
//    ticker: &'a str,
//    stock_exchange: &'a str
//) -> Stock {
//    use schema::stocks;
//    use schema::stocks::dsl::*;
//    let new_stock = InsertableStock {
//        ticker: ticker,
//        stock_exchange: stock_exchange,
//    };
//    diesel::insert_into(stocks::table)
//        .values(&new_stock)
//        .get_result(conn)
//        .expect("Error saving new stock")
//}
