use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::thread;
//TODO: Integrate this into the API by making a Manual trigger via web request w/ secret key
#[derive(Deserialize)]
struct StockInfo {
    stocks: Vec<Stock>,
}

#[derive(Deserialize)]
struct Stock {
    ticker: Option<String>,
    short_description: Option<String>,
    long_description: Option<String>,
    name: Option<String>,
    sic: Option<String>,
    legal_name: Option<String>,
    stock_exchange: Option<String>,
}

#[derive(Serialize)]
struct TokenCountedStockInfo {
    stocks: Vec<TokenCountedStock>,
}

#[derive(Serialize)]
struct TokenCountedStock {
    ticker: String,
    name: String,
    legal_name: String,
    sic: String,
    stock_exchange: String,
    token_count: HashMap<String, u32>,
}

pub fn generate_token_counted_stock_file(filename: String) -> Result<()> {
    println!("Deserializing json for preprocessing");
    let stock_info: StockInfo = parse_stocks_from_json(filename);
    println!("Generating token count dictionary");
    let token_counted_stock_info: TokenCountedStockInfo = count_tokens(stock_info);
    println!("Writing preprocessed json to file");
    write_token_counted_stocks_to_file(token_counted_stock_info);
    Ok(())
}
fn write_token_counted_stocks_to_file(token_counted_stock_info: TokenCountedStockInfo) {
    let string_stocks = serde_json::to_string(&token_counted_stock_info)
        .expect("Failed to serialize token counted stocks");
    let new_filename = String::from("TokenCountedStocks.json");
    fs::write(new_filename, string_stocks).expect("Failed to write to new token counted file");
}

fn parse_stocks_from_json(filename: String) -> StockInfo {
    let raw_json_string: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let stock_vector: Vec<Stock> =
        serde_json::from_str(&raw_json_string).expect("Could not parse file");
    StockInfo {
        stocks: stock_vector,
    }
}

fn count_tokens(stock_info: StockInfo) -> TokenCountedStockInfo {
    let mut token_counted_stocks: Vec<TokenCountedStock> = Vec::new();
    let thread_count = 16; //TODO: Pull this in dynamically - just use number of cores
    let mut thread_pool: Vec<thread::JoinHandle<Vec<TokenCountedStock>>> = Vec::new();
    let sharable_stock_info: Arc<StockInfo> = Arc::new(stock_info);
    for index in 0..thread_count {
        let skip_index = (sharable_stock_info.stocks.len() / thread_count) * index;
        let sharable_stock_info: Arc<StockInfo> = Arc::clone(&sharable_stock_info);
        let join_handle = thread::spawn(move || {
            let stock_info_thread_chunk = sharable_stock_info
                .stocks
                .iter()
                .skip(skip_index)
                .take(sharable_stock_info.stocks.len() / thread_count);

            let mut token_counted_batch: Vec<TokenCountedStock> = Vec::new();
            for stock in stock_info_thread_chunk {
                let token_map: HashMap<String, u32> = HashMap::new();
                if let Some(description) = &stock.long_description {
                    if description.to_ascii_lowercase().trim() != "n/a" {
                        token_counted_batch.push(generate_token_counted_stock(
                            &description.to_ascii_lowercase(),
                            token_map,
                            stock,
                        ));
                    }
                } else if let Some(description) = &stock.short_description {
                    if description.to_ascii_lowercase().trim() != "n/a" {
                        token_counted_batch.push(generate_token_counted_stock(
                            &description.to_ascii_lowercase(),
                            token_map,
                            stock,
                        ));
                    }
                }
            }

            token_counted_batch
        });
        thread_pool.push(join_handle);
    }

    for handle in thread_pool {
        token_counted_stocks.append(&mut handle.join().unwrap());
    }

    TokenCountedStockInfo {
        stocks: token_counted_stocks,
    }
}

fn generate_token_counted_stock(
    description: &String,
    mut token_map: HashMap<String, u32>,
    stock: &Stock,
) -> TokenCountedStock {
    for token in description.split(' ') {
        if !STOP_WORDS.contains(&token) {
            let alpha_numeric_token: String = token
                .chars()
                .filter(|character| character.is_ascii_alphanumeric())
                .collect();
            if token_map.contains_key(&alpha_numeric_token) {
                let current_count = &token_map
                    .get(&alpha_numeric_token)
                    .expect("Retrieval triggered on empty map entry");
                let new_count = *current_count + 1;
                &token_map.insert(alpha_numeric_token, new_count);
            } else {
                token_map.insert(String::from(alpha_numeric_token), 1);
            }
        }
    }
    let token_counted_stock = TokenCountedStock {
        ticker: match &stock.ticker {
            Some(ticker) => ticker.clone(),
            None => String::from(""),
        },
        name: match &stock.name {
            Some(name) => name.clone(),
            None => String::from(""),
        },
        legal_name: match &stock.legal_name {
            Some(legal_name) => legal_name.clone(),
            None => String::from(""),
        },
        sic: match &stock.sic {
            Some(sic) => sic.clone(),
            None => String::from(""),
        },
        stock_exchange: match &stock.stock_exchange {
            Some(stock_exchange) => stock_exchange.clone(),
            None => String::from(""),
        },
        token_count: token_map,
    };
    token_counted_stock
}

static STOP_WORDS: [&str; 543] = [
    "a's",
    "able",
    "about",
    "above",
    "according",
    "accordingly",
    "across",
    "actually",
    "after",
    "afterwards",
    "again",
    "against",
    "ain't",
    "all",
    "allow",
    "allows",
    "almost",
    "alone",
    "along",
    "already",
    "also",
    "although",
    "always",
    "am",
    "among",
    "amongst",
    "an",
    "and",
    "another",
    "any",
    "anybody",
    "anyhow",
    "anyone",
    "anything",
    "anyway",
    "anyways",
    "anywhere",
    "apart",
    "appear",
    "appreciate",
    "appropriate",
    "are",
    "aren't",
    "around",
    "as",
    "aside",
    "ask",
    "asking",
    "associated",
    "at",
    "available",
    "away",
    "awfully",
    "be",
    "became",
    "because",
    "become",
    "becomes",
    "becoming",
    "been",
    "before",
    "beforehand",
    "behind",
    "being",
    "believe",
    "below",
    "beside",
    "besides",
    "best",
    "better",
    "between",
    "beyond",
    "both",
    "brief",
    "but",
    "by",
    "c'mon",
    "c's",
    "came",
    "can",
    "can't",
    "cannot",
    "cant",
    "cause",
    "causes",
    "certain",
    "certainly",
    "changes",
    "clearly",
    "co",
    "com",
    "come",
    "comes",
    "concerning",
    "consequently",
    "consider",
    "considering",
    "contain",
    "containing",
    "contains",
    "corresponding",
    "could",
    "couldn't",
    "course",
    "currently",
    "definitely",
    "described",
    "despite",
    "did",
    "didn't",
    "different",
    "do",
    "does",
    "doesn't",
    "doing",
    "don't",
    "done",
    "down",
    "downwards",
    "during",
    "each",
    "edu",
    "eg",
    "eight",
    "either",
    "else",
    "elsewhere",
    "enough",
    "entirely",
    "especially",
    "et",
    "etc",
    "even",
    "ever",
    "every",
    "everybody",
    "everyone",
    "everything",
    "everywhere",
    "ex",
    "exactly",
    "example",
    "except",
    "far",
    "few",
    "fifth",
    "first",
    "five",
    "followed",
    "following",
    "follows",
    "for",
    "former",
    "formerly",
    "forth",
    "four",
    "from",
    "further",
    "furthermore",
    "get",
    "gets",
    "getting",
    "given",
    "gives",
    "go",
    "goes",
    "going",
    "gone",
    "got",
    "gotten",
    "greetings",
    "had",
    "hadn't",
    "happens",
    "hardly",
    "has",
    "hasn't",
    "have",
    "haven't",
    "having",
    "he",
    "he's",
    "hello",
    "help",
    "hence",
    "her",
    "here",
    "here's",
    "hereafter",
    "hereby",
    "herein",
    "hereupon",
    "hers",
    "herself",
    "hi",
    "him",
    "himself",
    "his",
    "hither",
    "hopefully",
    "how",
    "howbeit",
    "however",
    "i'd",
    "i'll",
    "i'm",
    "i've",
    "ie",
    "if",
    "ignored",
    "immediate",
    "in",
    "inasmuch",
    "inc",
    "indeed",
    "indicate",
    "indicated",
    "indicates",
    "inner",
    "insofar",
    "instead",
    "into",
    "inward",
    "is",
    "isn't",
    "it",
    "it'd",
    "it'll",
    "it's",
    "its",
    "itself",
    "just",
    "keep",
    "keeps",
    "kept",
    "know",
    "known",
    "knows",
    "last",
    "lately",
    "later",
    "latter",
    "latterly",
    "least",
    "less",
    "lest",
    "let",
    "let's",
    "like",
    "liked",
    "likely",
    "little",
    "look",
    "looking",
    "looks",
    "ltd",
    "mainly",
    "many",
    "may",
    "maybe",
    "me",
    "mean",
    "meanwhile",
    "merely",
    "might",
    "more",
    "moreover",
    "most",
    "mostly",
    "much",
    "must",
    "my",
    "myself",
    "name",
    "namely",
    "nd",
    "near",
    "nearly",
    "necessary",
    "need",
    "needs",
    "neither",
    "never",
    "nevertheless",
    "new",
    "next",
    "nine",
    "no",
    "nobody",
    "non",
    "none",
    "noone",
    "nor",
    "normally",
    "not",
    "nothing",
    "novel",
    "now",
    "nowhere",
    "obviously",
    "of",
    "off",
    "often",
    "oh",
    "ok",
    "okay",
    "old",
    "on",
    "once",
    "one",
    "ones",
    "only",
    "onto",
    "or",
    "other",
    "others",
    "otherwise",
    "ought",
    "our",
    "ours",
    "ourselves",
    "out",
    "outside",
    "over",
    "overall",
    "own",
    "particular",
    "particularly",
    "per",
    "perhaps",
    "placed",
    "please",
    "plus",
    "possible",
    "presumably",
    "probably",
    "provides",
    "que",
    "quite",
    "qv",
    "rather",
    "rd",
    "re",
    "really",
    "reasonably",
    "regarding",
    "regardless",
    "regards",
    "relatively",
    "respectively",
    "right",
    "said",
    "same",
    "saw",
    "say",
    "saying",
    "says",
    "second",
    "secondly",
    "see",
    "seeing",
    "seem",
    "seemed",
    "seeming",
    "seems",
    "seen",
    "self",
    "selves",
    "sensible",
    "sent",
    "serious",
    "seriously",
    "seven",
    "several",
    "shall",
    "she",
    "should",
    "shouldn't",
    "since",
    "six",
    "so",
    "some",
    "somebody",
    "somehow",
    "someone",
    "something",
    "sometime",
    "sometimes",
    "somewhat",
    "somewhere",
    "soon",
    "sorry",
    "specified",
    "specify",
    "specifying",
    "still",
    "sub",
    "such",
    "sup",
    "sure",
    "t's",
    "take",
    "taken",
    "tell",
    "tends",
    "th",
    "than",
    "thank",
    "thanks",
    "thanx",
    "that",
    "that's",
    "thats",
    "the",
    "their",
    "theirs",
    "them",
    "themselves",
    "then",
    "thence",
    "there",
    "there's",
    "thereafter",
    "thereby",
    "therefore",
    "therein",
    "theres",
    "thereupon",
    "these",
    "they",
    "they'd",
    "they'll",
    "they're",
    "they've",
    "think",
    "third",
    "this",
    "thorough",
    "thoroughly",
    "those",
    "though",
    "three",
    "through",
    "throughout",
    "thru",
    "thus",
    "to",
    "together",
    "too",
    "took",
    "toward",
    "towards",
    "tried",
    "tries",
    "truly",
    "try",
    "trying",
    "twice",
    "two",
    "un",
    "under",
    "unfortunately",
    "unless",
    "unlikely",
    "until",
    "unto",
    "up",
    "upon",
    "us",
    "use",
    "used",
    "useful",
    "uses",
    "using",
    "usually",
    "value",
    "various",
    "very",
    "via",
    "viz",
    "vs",
    "want",
    "wants",
    "was",
    "wasn't",
    "way",
    "we",
    "we'd",
    "we'll",
    "we're",
    "we've",
    "welcome",
    "well",
    "went",
    "were",
    "weren't",
    "what",
    "what's",
    "whatever",
    "when",
    "whence",
    "whenever",
    "where",
    "where's",
    "whereafter",
    "whereas",
    "whereby",
    "wherein",
    "whereupon",
    "wherever",
    "whether",
    "which",
    "while",
    "whither",
    "who",
    "who's",
    "whoever",
    "whole",
    "whom",
    "whose",
    "why",
    "will",
    "willing",
    "wish",
    "with",
    "within",
    "without",
    "won't",
    "wonder",
    "would",
    "wouldn't",
    "yes",
    "yet",
    "you",
    "you'd",
    "you'll",
    "you're",
    "you've",
    "your",
    "yours",
    "yourself",
    "yourselves",
    "zero",
];
