use super::super::repos::token_count_repo::TokenCountRepo;
use super::super::repos::similarity_ranking_repo::SimilarityRankingRepo;
use super::super::repos::stock_repo::StockRepo;
use std::sync::Arc;
use diesel::PgConnection;
use crate::repos::stock_repo::stock_models::QueryableStock;
use crate::repos::token_count_repo::token_count_models::QueryableTokenCount;
use std::collections::HashMap;
use crate::services::stock_similarity_service::ranker::{TokenCountedStock, TokenCountedStockInfo, RankedStock};

pub mod ranker;

pub struct StockSimilarityService{
    stock_repo: Arc<StockRepo>,
    similarity_ranking_repo: Arc<SimilarityRankingRepo>,
    token_count_repo: Arc<TokenCountRepo>
}
impl StockSimilarityService {
    pub fn new<'a>(injected_stock_repo: Arc<StockRepo>,
           injected_similarity_ranking_repo: Arc<SimilarityRankingRepo>,
           injected_token_count_repo: Arc<TokenCountRepo>) -> StockSimilarityService {
        StockSimilarityService {
            stock_repo: injected_stock_repo,
            similarity_ranking_repo: injected_similarity_ranking_repo,
            token_count_repo: injected_token_count_repo
        }
    }

    pub fn get_all_stocks(&self,db_conn: &PgConnection) -> Vec<QueryableStock> {
        self.stock_repo.get_all(db_conn)
    }

    pub fn get_stock_by_ticker(&self, db_conn: &PgConnection, ticker: String) -> Option<QueryableStock> {
        self.stock_repo.get_one_by_pk(db_conn, ticker)
    }

    pub fn get_all_token_counts(&self, db_conn: &PgConnection) -> Vec<QueryableTokenCount> {
        self.token_count_repo.get_all(db_conn)
    }

    pub fn get_similar_stocks(&self, db_conn: &PgConnection, ticker: String) -> Result<Vec<RankedStock>, serde_json::Error>{
        let token_counts = self.token_count_repo.get_all(db_conn);
        let mut token_count_map: HashMap<String, HashMap<String,i32>> = HashMap::new();
        token_counts.iter().for_each(|token_count|{
            match token_count_map.get_mut(token_count.ticker.as_str()) {
                Some(stock_token_count) => {
                    stock_token_count.insert(token_count.token.clone(),token_count.count.clone());
                },
                None => {
                    let mut newHashMap = HashMap::new();
                    newHashMap.insert(token_count.token.clone(),token_count.count.clone());
                    token_count_map.insert(token_count.ticker.clone(), newHashMap);
                }
            };
        });
        let stocks = self.stock_repo.get_all(db_conn);
        let mut first = true;
        let token_counted_stocks = TokenCountedStockInfo {
            stocks: stocks.iter().filter(|stock|{
                match &stock.stock_exchange {
                    Some(exchange)=>exchange.eq("NYSE"),
                    None => false
                }
            })
              .filter(|stock|{
                    match token_count_map.get(stock.ticker.as_str()) {
                        Some(_)=>true,
                        None=>false
                    }
              }).map(|stock|{
                    let stock_token_count_map = token_count_map.get(stock.ticker.as_str()).expect("Filter failure");
                    TokenCountedStock {
                        ticker: stock.ticker.clone(),
                        stock_exchange: stock.stock_exchange.as_ref().expect("stock had no exchange.").clone(),
                        token_count: stock_token_count_map.clone(),
                    }
              }).collect()
        };
        ranker::generate_ranking(ticker,token_counted_stocks)

    }
}