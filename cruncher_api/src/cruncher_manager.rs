use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::repos;
use repos::stock_repo::StockRepo;
use repos::similarity_ranking_repo::SimilarityRankingRepo;
use repos::token_count_repo::TokenCountRepo;

use crate::services;
use services::stock_similarity_service::StockSimilarityService;
use std::rc::Rc;
use std::sync::Arc;


fn establish_db_connection(database_url: &String)-> PgConnection {
    PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
}


pub struct CruncherManager {
    stock_repo: Arc<StockRepo>,
    similarity_ranking_repo: Arc<SimilarityRankingRepo>,
    token_count_repo: Arc<TokenCountRepo>,
    pub stock_similarity_service: Arc<StockSimilarityService>,
}
impl CruncherManager {
    pub fn new() -> CruncherManager{
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let stock_repo = Arc::new(StockRepo::new());
        let similarity_ranking_repo = Arc::new(SimilarityRankingRepo::new());
        let token_count_repo = Arc::new(TokenCountRepo::new());
        let stock_similarity_service = Arc::new(StockSimilarityService::new(Arc::clone(&stock_repo),
                                                                                             Arc::clone(&similarity_ranking_repo),
                                                                                             Arc::clone(&token_count_repo)));

        CruncherManager {
            stock_repo,
            similarity_ranking_repo,
            token_count_repo,
            stock_similarity_service
        }
    }
}