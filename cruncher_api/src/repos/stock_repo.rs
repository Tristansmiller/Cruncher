pub mod stock_models;
use self::stock_models::{InsertableStock, QueryableStock};
use super::super::diesel::prelude::*;
use super::super::logger::{LogLevel, Logger};
use super::super::schema::stocks;
use super::super::schema::stocks::dsl::*;
use diesel::pg::PgConnection;
use diesel::result::Error;

pub struct StockRepo {
    logger: Logger,
}
impl StockRepo {
    pub fn new() -> StockRepo {
        StockRepo {
            logger: Logger::new("repos.stock_repos.rs"),
        }
    }

    pub fn get_one_by_pk(&self, db_conn: &PgConnection, search_pk: String) -> Option<QueryableStock> {
        let result = stocks.find(search_pk).first(db_conn);
        match result {
            Ok(result_val) => Some(result_val),
            Err(err) => {
                self.logger
                    .log("Failed to retrieve stocks", LogLevel::Warning);
                None
            }
        }
    }
    pub fn get_all(&self, db_conn: &PgConnection,) -> Vec<QueryableStock> {
        let results: Vec<QueryableStock> = stocks
            .load::<QueryableStock>(db_conn)
            .expect("Error loading stocks.");
        results
    }

    pub fn get_vec_by_ids(&self, db_conn: &PgConnection, search_vals: Vec<String>) -> Vec<QueryableStock> {
        let results: Vec<QueryableStock> = stocks
            .filter(ticker.eq_any(search_vals))
            .get_results(db_conn)
            .expect("Error loading stocks.");
        results
    }

    pub fn save_one(&self, db_conn: &PgConnection, save_val: InsertableStock) -> Result<QueryableStock, Error> {
        let already_existing_result: Result<QueryableStock, Error> = stocks.find(save_val.ticker.to_string()).first(db_conn);
        let result = match already_existing_result {
            Ok(already_existing_val) => diesel::update(&already_existing_val)
                                                .set(&save_val)
                                                .get_result(db_conn),
            Err(err) => {
                diesel::insert_into(stocks::table)
                    .values(save_val)
                    .get_result(db_conn)
            }
        };
        result
    }
}
