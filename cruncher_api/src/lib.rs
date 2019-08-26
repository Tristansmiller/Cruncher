#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate dotenv;
extern crate proc_macro;
extern crate bigdecimal;
extern crate serde_json;
extern crate serde;


pub mod logger;
pub mod parser;
pub mod repos;
pub mod services;
pub mod schema;
pub mod endpoints;

use std::sync::Arc;

use repos::stock_repo::StockRepo;
use repos::similarity_ranking_repo::SimilarityRankingRepo;
use repos::token_count_repo::TokenCountRepo;
use services::stock_similarity_service::StockSimilarityService;
use endpoints::stock_similarity_endpoints;

#[database("cruncher_db")]
pub struct CruncherDbConn(diesel::PgConnection);

pub fn initialize_cruncher_api(){
    //Initialize Repositories
    let stock_repo = Arc::new(StockRepo::new());
    let similarity_ranking_repo = Arc::new(SimilarityRankingRepo::new());
    let token_count_repo = Arc::new(TokenCountRepo::new());

    //Initialize Services
    let stock_similarity_service = StockSimilarityService::new(Arc::clone(&stock_repo),
                                                               Arc::clone(&similarity_ranking_repo),
                                                               Arc::clone(&token_count_repo));

    //Connect dependencies (services, DB connection) to rocket and mount endpoints
    rocket::ignite()
        .manage(stock_similarity_service)
        .attach(CruncherDbConn::fairing())
        .mount("/", routes![
                                              stock_similarity_endpoints::get_all,
                                              stock_similarity_endpoints::get_stock_by_id,
                                              stock_similarity_endpoints::get_similar,
                                              stock_similarity_endpoints::get_similar_with_limit
                                            ])
        .launch();
}