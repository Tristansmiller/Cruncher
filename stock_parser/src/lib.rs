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

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod parser;
pub mod ranker;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{NewStock, Stock};
pub fn create_stock<'a>(
    conn: &PgConnection,
    ticker: &'a str,
    stock_exchange: &'a str,
    token_map: &'a str,
) -> Stock {
    use schema::stocks;

    let new_stock = NewStock {
        ticker: ticker,
        stockexchange: stock_exchange,
        tokenmap: token_map,
    };

    diesel::insert_into(stocks::table)
        .values(&new_stock)
        .get_result(conn)
        .expect("Error saving new stock")
}
