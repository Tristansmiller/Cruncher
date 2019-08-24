use super::super::repos::token_count_repo::TokenCountRepo;
use super::super::repos::similarity_ranking_repo::SimilarityRankingRepo;
use super::super::repos::stock_repo::StockRepo;
use std::sync::Arc;
use diesel::PgConnection;
use crate::repos::stock_repo::stock_models::QueryableStock;
use crate::repos::token_count_repo::token_count_models::QueryableTokenCount;

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
}