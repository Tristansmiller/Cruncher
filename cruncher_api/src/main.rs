#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// use std::env;
// use std::process;
// use std::time;
// use cruncher_api::Config;
extern crate diesel;
extern crate cruncher_api;
pub mod cruncher_manager;
use self::diesel::prelude::*;
use self::models::*;
use self::cruncher_api::*;

#[get("/")]
fn index() -> &'static str {
    use cruncher_api::schema::stocks::dsl::*;
    let connection = establish_connection();
    let results = stocks
        .filter(stockexchange.eq("NasdaqGM"))
        .limit(5)
        .load::<Stock>(&connection)
        .expect("Error loading stocks");
    serde_json::to_string(&results).or_else(())
}

fn main() {
    //Rocket code
    rocket::ignite().mount("/", routes![index]).launch();

    //TODO: Build out SQL DB to store TokenCountedStock
    //TODO: Build out Diesel to retrieve TokenCountedStock
    //TODO: Add Rocket endpoint that does the processing and returns ranking JSON
    //Diesel code

    //println!("What would you like the ticker to be?");
    // let tickervar = "LMT";
    // let stock_exchangevar = "NYSE";
    // let tokenMapvar = "tokenmap";

    // let ticker2Var = "WMT";
    // let stock_exchange2Var = "NasdaqGM";
    // let tokenMap2Var = "tokenMap";

    //    let stock1 = create_stock(&connection, tickervar, stock_exchangevar, tokenMapvar);
    //    let stock2 = create_stock(&connection, ticker2Var, stock_exchange2Var, tokenMap2Var);

    //        println!("Displaying {} stocks", results.len());
    //        for stock in results {
    //            println!("{}", stock.ticker);
    //            println!("----------------");
    //            println!("{}", stock.stockexchange);
    //        }
}

// fn main() {
//     // let time = time::SystemTime::now();
//     // let config = Config::new(env::args()).unwrap_or_else(|err|{
//     //     println!("Problem parsing arguments: {}", err);
//     //     process::exit(1);
//     // });

//     // if let Err(e) = cruncher_api::run(config) {
//     //     println!("Application error: {}", e);

//     //     process::exit(1);
//     // }
//     // println!("Successfully generated preprocessed data file - Total runtime: {}ms",time.elapsed().unwrap().as_millis());

// }
