-- Your SQL goes here

CREATE TABLE Stocks (
  ticker VARCHAR PRIMARY KEY,
  stockExchange VARCHAR NOT NULL,
  tokenMap VARCHAR NOT NULL
)