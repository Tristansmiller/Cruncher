#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// use std::env;
// use std::process;
// use std::time;
// use cruncher_api::Config;
extern crate diesel;
extern crate cruncher_api;
extern crate serde_json;
extern crate serde;

pub mod cruncher_manager;
pub mod endpoints;

use rocket::State;
use self::cruncher_api::*;
use self::cruncher_manager::CruncherManager;
use endpoints::stock_similarity_endpoints;
pub mod db_refresh;
#[database("cruncher_db")]
pub struct CruncherDbConn(diesel::PgConnection);

//TODO: Figure out how to return 404's and other error codes

fn main() {
    //Rocket code
    let cruncher_manager = CruncherManager::new();
    rocket::ignite()
            .manage(cruncher_manager)
            .attach(CruncherDbConn::fairing())
            .mount("/", routes![
                                              stock_similarity_endpoints::get_all,
                                              stock_similarity_endpoints::get_stock_by_id
                                            ])
            .launch();



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
