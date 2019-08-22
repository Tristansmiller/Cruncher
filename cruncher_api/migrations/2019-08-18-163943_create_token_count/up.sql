-- Your SQL goes here
CREATE TABLE token_count (
    ticker VARCHAR NOT NULL REFERENCES stocks(ticker),
    token VARCHAR NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY(ticker, token)
)