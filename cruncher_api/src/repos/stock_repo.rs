pub mod stock_models;
use super::super::diesel::prelude::*;
use diesel::pg::PgConnection;
use super::super::schema::stocks;
use super::super::schema::stocks::dsl::*;
use self::stock_models::{QueryableStock, InsertableStock};
use diesel::result::Error;
use super::super::logger::{Logger, LogLevel};

pub struct StockRepo<'a>{
    db_connection : &'a PgConnection,
    logger: Logger
}
impl StockRepo<'_> {
    pub fn new(injected_db_connection :&PgConnection) -> StockRepo {
        StockRepo {
            db_connection: injected_db_connection,
            logger: Logger::new("repos.stock_repos.rs")
        }
    }
    pub fn get_one_by_ticker(&self, search_ticker: String) -> Option<QueryableStock> {
        let result_stock = stocks.find(search_ticker).first(self.db_connection);
        match result_stock {
            Ok(returned_stock)=>Some(returned_stock),
            Err(err)=>{
                self.logger.log("Failed to retrieve stocks", LogLevel::Warning);
                None
            }
        }
    }
    pub fn get_all(&self) -> Vec<QueryableStock> {
        let result_stocks: Vec<QueryableStock> = stocks.load::<QueryableStock>(self.db_connection)
                                                       .expect("Error loading stocks.");
        result_stocks
    }

    pub fn get_vec_by_ids(&self, search_tickers: Vec<String>) -> Vec<QueryableStock> {
        let result_stocks: Vec<QueryableStock> = stocks.filter(ticker.eq_any(search_tickers))
                                                       .get_results(self.db_connection)
                                                       .expect("Error loading stocks.");
        result_stocks
    }

    pub fn save_one(&self, save_stock: InsertableStock) -> Result<QueryableStock, Error> {
        let already_existing_stock : Option<QueryableStock> =
                                self.get_one_by_ticker(save_stock.ticker.to_string());
        let result = match already_existing_stock {
            Some(already_existing_stock) => {
                    diesel::update(&already_existing_stock).set(&save_stock)
                            .get_result(self.db_connection)
            },
            None => {
                diesel::insert_into(stocks::table)
                        .values(save_stock)
                        .get_result(self.db_connection)
            }
        };
        result
    }

//    pub fn save_vec(&self, save_stocks: Vec<InsertableStock>) -> Result<Vec<QueryableStock>, Error> {
//        let already_existing_stocks: Vec<QueryableStock> = self
//    }
}
