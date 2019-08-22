-- Your SQL goes here
CREATE TABLE similarity_ranking (
    tickerA VARCHAR NOT NULL REFERENCES stocks(ticker),
    tickerB VARCHAR NOT NULL REFERENCES stocks(ticker),
    similarity DECIMAL NOT NULL,
    PRIMARY KEY (tickerA, tickerB)
);
