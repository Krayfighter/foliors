
#![feature(slice_partition_dedup)]

use std::collections::HashMap;

use foliors::holdings::*;

const HISTORY: &str = "timeline.ron";
const HOLDINGS: &str = "current.ron";

#[derive(serde::Deserialize, Debug, Default)]
struct CoinPrices (pub HashMap<String, HashMap<String, f64>>);

fn main () -> Result<(), Box<dyn std::error::Error>> {
    println!("starting daemon");

    loop {
        let client = reqwest::blocking::Client::new();
        let mut history = holding::Records::from_file(HISTORY)?;
        let holdings = holding::Holdings::from_file(HOLDINGS)?;
    
        let mut query_string = "https://api.coingecko.com/api/v3/simple/price?vs_currencies=usd&ids=".to_owned();
    
        holdings.0.iter()
            .map(|holding| holding.asset.clone())
            .fold(vec!(), |mut acc, asset| {
                if !acc.contains(&asset) { acc.push(asset); }
                return acc;
            })
            .into_iter()
            .for_each(|asset| {
                use holding::Asset::*;
                match asset {
                    XMR => { query_string += "monero,"; }
                    BTC => { query_string += "bitcoin,"; }
                    ETH => { query_string += "ethereum,"; }
                    USD => {}
                }
            });
    
        let resp = client.get(query_string.as_str())
            .send()?
            .json::<CoinPrices>()?;
    
        let mut prices = resp.0.iter()
            .map(|(key, value)| {
                let price = *value.get("usd").unwrap();
                let asset = match key.as_str() {
                    "monero" => holding::Asset::XMR,
                    "ethereum" => holding::Asset::ETH,
                    "bitcoin" => holding::Asset::BTC,
                    other => panic!("unimplemented or unknown coin: {}", other)
                };
                return (asset, price);
            })
            .collect::<HashMap<holding::Asset, f64>>();
    
        prices.insert(holding::Asset::USD, 1.);
    
        history.push(holding::Record::new(holdings, prices));
    
        history.to_file(HISTORY)?;
    
        println!("write to history");
        println!("{:#?}", resp);

        // 8640 = (24*60*60)/10 = runs 10 times per day
        std::thread::sleep(std::time::Duration::from_secs(8640))
    }
}


