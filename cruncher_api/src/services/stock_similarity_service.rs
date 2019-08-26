use super::super::repos::token_count_repo::TokenCountRepo;
use super::super::repos::similarity_ranking_repo::SimilarityRankingRepo;
use super::super::repos::stock_repo::StockRepo;
use std::sync::Arc;
use diesel::PgConnection;
use serde::Serialize;
use crate::repos::stock_repo::stock_models::QueryableStock;
use crate::repos::token_count_repo::token_count_models::QueryableTokenCount;
use std::collections::HashMap;
use crate::services::stock_similarity_service::ranker::{TokenCountedStock, TokenCountedStockInfo, RankedStock, RankedResults};
use crate::repos::similarity_ranking_repo::similarity_ranking_models::InsertableSimilarityRanking;

pub mod ranker;
#[derive(Serialize)]
pub struct StockSimilarityItemDto {
    ticker: String,
  //  stock_exchange: String,
    ranking: i32
}
#[derive(Serialize)]
pub struct StockSimilarityTargetDto {
    ticker: String,
    //stock_exchange: String
}
#[derive(Serialize)]
pub struct StockSimilarityResultDto {
    ranked_stocks:Vec<StockSimilarityItemDto>,
    target_stock: StockSimilarityTargetDto
}
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

    fn create_token_count_maps(&self, db_conn: &PgConnection) -> HashMap<String, HashMap<String, i32>> {
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
        token_count_map
    }

    //TODO: This takes longer than the actual ranking. Figure out a smarter way to do this, maybe you can get fancy with the SQL joins to get the data closer to the needed format
    fn create_token_counted_stocks(&self, db_conn: &PgConnection) -> TokenCountedStockInfo {
        let token_count_map = self.create_token_count_maps(db_conn);
        let stocks = self.stock_repo.get_all(db_conn);
        let mut first = true;
        TokenCountedStockInfo {
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
        }
    }
    //TODO: Figure out inner joins so we can get back stocks with additional info from the sim ranking repo
    pub fn get_similar_stocks(&self, db_conn: &PgConnection, ticker: String, limit: i32) -> Result<StockSimilarityResultDto, serde_json::Error>{
        let cached_rankings = self.similarity_ranking_repo.get_rankings_by_target_stock(db_conn, ticker.clone(), limit);
        if cached_rankings.len() > 0 {
            println!("Found cached data.");
            let target = StockSimilarityTargetDto {
                ticker: cached_rankings.get(0).expect("Malformed similarity ranking data.").tickera.clone()
            };
            let results = cached_rankings.iter().map(|cached_result|{
                StockSimilarityItemDto {
                    ticker: cached_result.tickerb.clone(),
                    ranking: cached_result.similarity
                }
            }).collect();
            println!("Done");
            Ok(StockSimilarityResultDto {
                target_stock: target,
                ranked_stocks: results
            })
        } else {
            let token_counted_stocks = self.create_token_counted_stocks(db_conn);
            let results = ranker::generate_ranking(ticker,token_counted_stocks, limit);
            println!("Done");
            match results {
                Ok(ranked_results) => {
                    ranked_results.ranked_stocks.iter().for_each(|ranked_stock|{
                        self.similarity_ranking_repo.save_one(db_conn,  InsertableSimilarityRanking{
                            tickera: ranked_results.target_stock.ticker.as_str(),
                            tickerb: ranked_stock.token_counted_stock.ticker.as_str(),
                            similarity: &((ranked_stock.ranking * 10000.0) as i32)
                        });
                    });
                    Ok(StockSimilarityResultDto{
                        ranked_stocks: ranked_results.ranked_stocks.iter().map(|ranked_stock|{
                            StockSimilarityItemDto {
                                ticker: ranked_stock.token_counted_stock.ticker.clone(),
                                ranking: (ranked_stock.ranking * 10000.0) as i32
                            }
                        }).collect(),
                        target_stock: StockSimilarityTargetDto {
                            ticker: ranked_results.target_stock.ticker,
                        }
                    })
                },
                Err(e) => Err(e)
            }
        }

    }
}