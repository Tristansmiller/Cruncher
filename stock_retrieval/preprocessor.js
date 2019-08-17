var TokenCountedStock = require('./TokenCountedStocks.json');
const fs = require('fs');

var filteredJSON = TokenCountedStock.stocks.filter(stock=>{
    return stock.stock_exchange === "NasdaqGM" || stock.stock_exchange === "NYSE";
}).map(stock=> {
    return {
        ticker: stock.ticker,
        stock_exchange: stock.stock_exchange,
        token_count: stock.token_count,
    };
});

filteredJSON = {stocks: filteredJSON};

fs.writeFile('filtered_stocks_NasdaqGM_NYSE.json',JSON.stringify(filteredJSON),(err)=>{
    if( err ) {
        console.log(err);
    } else {
        console.log('success');
    }    
});

