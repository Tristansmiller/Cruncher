var intrinioSDK = require('intrinio-sdk');
intrinioSDK.ApiClient.instance.authentications['ApiKeyAuth'].apiKey = "OmRjYTBmNmQxMmI3MmRlMmExN2Q5ZTIzNTVlYWE0NDNk";

var companyAPI = new intrinioSDK.CompanyApi();
async function fetchStuff() {
    var opts = { 
        'latestFilingDate': null, // Date | Last filing date
        'sic': null, // String | Return companies with the given Standard Industrial Classification code
        'template': null, // String | Return companies with the given financial statement template
        'sector': null, // String | Return companies in the given industry sector
        'industryCategory': null, // String | Return companies in the given industry category
        'industryGroup': null, // String | Return companies in the given industry group
        'hasFundamentals': true, // Boolean | Return only companies that have fundamentals when true
        'hasStockPrices': true, // Boolean | Return only companies that have stock prices when true
        'pageSize': 100, // Number | The number of results to return
        'nextPage': null // String | Gets the next page of data from a previous API call
      };
      let tickerArr = [];
      let companyArr = [];
      let data = await companyAPI.getAllCompanies(opts);
      data.companies.forEach(company=>tickerArr.push(company.ticker));
      opts.nextPage = data.next_page;
      while(data.companies.next_page!=null){
      data = await companyAPI.getAllCompanies(opts);
      data.companies.forEach(company=>tickerArr.push(company.ticker));
      opts.nextPage = data.next_page;
      }
      tickerArr.forEach(ticker=>{
          companyAPI.getCompany(ticker).then(function(data){
          companyArr.push(data);
          console.log(companyArr);
          });
      });
}
fetchStuff();
