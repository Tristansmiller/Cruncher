use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::NAN;
use std::fs;
use std::hash::{Hash, Hasher};


#[derive(Deserialize, Clone, Debug)]
pub struct TokenCountedStockInfo {
    pub stocks: Vec<TokenCountedStock>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenCountedStock {
    pub ticker: String,
    pub stock_exchange: String,
    pub token_count: HashMap<String, i32>,
}

#[derive(Debug)]
struct Term {
    term: String,
    tf_idf_weight: f32,
    document_frequency: u16,
}

impl Term {
    fn new(term_str: String) -> Term {
        Term {
            term: term_str,
            tf_idf_weight: 0.0,
            document_frequency: 0,
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        self.term == other.term
    }
}

impl Eq for Term {}

impl Hash for Term {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.term.hash(state);
    }
}

#[derive(Debug)]
struct Query {
    ticker: String,
    terms: HashSet<Term>,
}

impl Query {
    fn new(ticker: String) -> Query {
        Query {
            ticker,
            terms: HashSet::new(),
        }
    }
}
#[derive(Serialize, Clone)]
pub struct RankedResults {
    pub ranked_stocks: Vec<RankedStock>,
    pub target_stock: TokenCountedStock
}
#[derive(Serialize, Clone)]
pub struct RankedStock {
    pub token_counted_stock: TokenCountedStock,
    pub ranking: f32,
}
#[derive(Debug)]
struct TFIDFWeightProduct {
    document_weight: f32,
    query_weight: f32,
    term: String,
}

impl PartialEq for TFIDFWeightProduct {
    fn eq(&self, other: &TFIDFWeightProduct) -> bool {
        self.term == other.term
    }
}

impl Eq for TFIDFWeightProduct {}

impl Hash for TFIDFWeightProduct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.term.hash(state);
    }
}

pub fn generate_ranking(ticker: String,token_counted_stocks: TokenCountedStockInfo) -> Result<RankedResults> {
  //  let token_counted_stocks: TokenCountedStockInfo = parse_token_counted_stocks_from_json(String::from("C:\\Users\\Trist\\RustLearning\\Stock Suggestion Engine\\cruncher_api\\filtered_stocks_NasdaqGM_NYSE.json"));
    let target_stock: &TokenCountedStock =
        match find_stock_with_matching_ticker(ticker, &token_counted_stocks) {
            Some(val) => val,
            None => panic!("Couldn't find a stock that matched the given ticker"),
        };
    let query: Query = initialize_query(target_stock, &token_counted_stocks);
    let rankings: Vec<RankedStock> = rank_documents_for_similarity(&query, token_counted_stocks.clone());
    output_rankings(&rankings);
    Ok(RankedResults {
        ranked_stocks: rankings,
        target_stock: target_stock.clone()
    })
}

fn output_rankings(rankings: &Vec<RankedStock>) {
    let mut counter = 1;
    for ranked_stock in rankings.iter().take(20) {
        println!(
            "{} - {} : {}",
            counter, ranked_stock.ranking, ranked_stock.token_counted_stock.ticker
        );
        counter += 1;
    }
}

fn parse_token_counted_stocks_from_json(filename: String) -> TokenCountedStockInfo {
    let nyse = String::from("NYSE");
    let nasdaqgs = String::from("NasdaqGS");
    let nasdaqgm = String::from("NasdaqGM");
    let nasdaqcm = String::from("NasdaqCM");
    let raw_json_string: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let token_counted_stocks: TokenCountedStockInfo =
        serde_json::from_str(&raw_json_string).expect("Could not parse file");
    let filtered_token_counted_stocks = token_counted_stocks
        .stocks
        .into_iter()
        .filter(|a| {
            a.stock_exchange == nyse
                || a.stock_exchange == nasdaqgs
                || a.stock_exchange == nasdaqgm
                || a.stock_exchange == nasdaqcm
        })
        .collect();
    TokenCountedStockInfo {
        stocks: filtered_token_counted_stocks,
    }
}

fn find_stock_with_matching_ticker(
    ticker: String,
    stock_info: &TokenCountedStockInfo,
) -> Option<&TokenCountedStock> {
    //TODO: Convert this to a binary search. If it's searching ~7000 stocks you could get a small boost
    let mut target_stock: Option<&TokenCountedStock> = None;
    for token_counted_stock in stock_info.stocks.iter() {
        if token_counted_stock.ticker == ticker {
            target_stock = Some(token_counted_stock);
            break;
        }
    }
    target_stock
}

fn initialize_query(query_target: &TokenCountedStock, documents: &TokenCountedStockInfo) -> Query {
    let mut query: Query = Query::new(query_target.ticker.to_string());
    let num_documents: usize = documents.stocks.len();
    for (_index, token) in query_target.token_count.keys().enumerate() {
        let mut term = Term::new(token.to_string());
        term.document_frequency = calculate_document_frequency(&token, &documents);
        let term_frequency = query_target
            .token_count
            .get(token)
            .expect("Hash map failure. Map key was unable to retrieve value.");
        term.tf_idf_weight =
            calculate_tf_idf_weight(*term_frequency, term.document_frequency, num_documents);
        query.terms.insert(term);
    }
    query
}

//TODO: Possibly make this function parallel to speed up token counting - evaluate if it's worthwhile despite being threaded out at a higher level e.g. does the impact from further threading increase the overhead of managing threads too much to make it worth the perfomance gain from this being concurrent
fn calculate_document_frequency(token: &String, documents: &TokenCountedStockInfo) -> u16 {
    let mut counter = 0;
    for document in documents.stocks.iter() {
        if document.token_count.contains_key(token) {
            counter += 1;
        }
    }
    counter
}

fn calculate_tf_idf_weight(
    term_frequency: i32,
    document_frequency: u16,
    num_documents: usize,
) -> f32 {
    let mut result: f32 = 0.0;
    if term_frequency != 0 && document_frequency != 0 {
        let f_term_frequency = term_frequency as f32;
        let f_document_frequency = document_frequency as f32;
        let f_num_documents = num_documents as f32;
        let document_ratio: f32 = f_num_documents / f_document_frequency;
        result = 1.0 + f_term_frequency.log10() + document_ratio.log10();
    }
    result
}

fn calculate_document_similarity(
    query: &Query,
    stock: &TokenCountedStock,
    num_documents: usize,
) -> f32 {
    let mut tf_idf_weight_products: HashSet<TFIDFWeightProduct> = HashSet::new();
    let mut magnitude: f32 = 0.0;
    for term in query.terms.iter() {
        let term_frequency = match stock.token_count.get(&term.term) {
            Some(val) => val,
            None => &(0 as i32),
        };
        let document_tf_idf: f32 =
            calculate_tf_idf_weight(*term_frequency, term.document_frequency, num_documents);
        magnitude += document_tf_idf * document_tf_idf;
        tf_idf_weight_products.insert(TFIDFWeightProduct {
            document_weight: document_tf_idf,
            query_weight: term.tf_idf_weight,
            term: term.term.to_string(),
        });
    }
    //TODO: Am I not actually calculating similarity? It looks like im just getting the vector, normalizing it and then takling the largest
    magnitude = magnitude.sqrt();
    let mut sum: f32 = 0.0;
    for weight_product in tf_idf_weight_products.into_iter() {
        sum += (weight_product.document_weight  * weight_product.query_weight) / magnitude;
    }
    sum
}

fn rank_documents_for_similarity(
    query: &Query,
    document_collection: TokenCountedStockInfo,
) -> Vec<RankedStock> {
    let num_documents = document_collection.stocks.len();
    let mut document_rankings: Vec<RankedStock> = Vec::new();
    for document in document_collection.stocks.iter() {
        if document.ticker != query.ticker {
            //TODO: This is probably the best spot to thread this out, should be ~linear scaling with threads
            let similarity_val = calculate_document_similarity(query, document, num_documents);
            document_rankings.push(RankedStock {
                ranking: similarity_val,
                token_counted_stock: document.clone(),
            });
        }
    }
    //TODO: Pull this out into a "sortRankedDocuments" and rename filtered_ranks to sorted_ranks
    let mut filtered_ranks: Vec<RankedStock> = document_rankings
        .into_iter()
        .filter(|a| a.ranking != NAN)
        .collect();
    filtered_ranks.sort_by(|a, b| {
        b.ranking
            .partial_cmp(&a.ranking)
            .unwrap_or_else(|| std::cmp::Ordering::Equal)
    });
    filtered_ranks[0..10].to_vec()
}
