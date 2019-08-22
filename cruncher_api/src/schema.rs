table! {
    similarity_ranking (tickera, tickerb) {
        tickera -> Varchar,
        tickerb -> Varchar,
        similarity -> Numeric,
    }
}

table! {
    stocks (ticker) {
        ticker -> Varchar,
        name -> Nullable<Varchar>,
        legal_name -> Nullable<Varchar>,
        sic -> Nullable<Varchar>,
        stock_exchange -> Nullable<Varchar>,
    }
}

table! {
    token_count (ticker, token) {
        ticker -> Varchar,
        token -> Varchar,
        count -> Int4,
    }
}

joinable!(token_count -> stocks (ticker));

allow_tables_to_appear_in_same_query!(
    similarity_ranking,
    stocks,
    token_count,
);
