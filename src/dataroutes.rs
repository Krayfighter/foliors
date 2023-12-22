

use crate::holdings::*;

pub const HOLDINGS: &str = "current.ron";
pub const HISTORY: &str = "timeline.ron";

pub async fn read_holdings() -> Result<holding::Holdings, String> {
    let thread_result = rocket::tokio::task::spawn_blocking(|| {
        return match holding::Holdings::from_file(HOLDINGS) {
            Ok(holdings) => Ok(holdings),
            Err(e) => Err(e.to_string()),
        };
    }).await;

    return match thread_result {
        Ok(res) => res,
        Err(e) => Err(e.to_string()),
    };
}

pub async fn read_history() -> Result<holding::Records, String> {
    let thread_result = rocket::tokio::task::spawn_blocking(|| {
        return match holding::Records::from_file(HISTORY) {
            Ok(history) => Ok(history),
            Err(e) => Err(e.to_string())
        }
    }).await;

    return match thread_result {
        Ok(res) => res,
        Err(e) => Err(e.to_string()),
    };
}


#[get("/history")]
pub async fn get_history_json() -> rocket::serde::json::Json<holding::Records> {
    return rocket::serde::json::Json(
        read_history().await.unwrap()
    );
}

#[post(
    "/register_prices",
    format = "application/json",
    data = "<json_prices>",
)]
pub async fn register_prices(json_prices: String) -> rocket::response::Redirect {

    use std::collections::HashMap as HashMap;

    #[derive(serde::Deserialize)]
    struct Prices (pub Vec<(holding::Asset, f64)>);

    let mut prices: HashMap<holding::Asset, f64> = HashMap::new();

    serde_json::from_str::<Prices>(&json_prices)
        .unwrap()
        .0
        .into_iter()
        .for_each(|(asset, price)| {
            prices.insert(asset, price);
        });

    let record = holding::Record::new(
        read_holdings().await.unwrap(),
        prices
    );

    let _ = rocket::tokio::task::spawn_blocking(move || {
        let mut history = holding::Records::from_file(HISTORY).unwrap();
        history.push(record);
        history.to_file(HISTORY);
    }).await.unwrap();

    return rocket::response::Redirect::to(uri!("/history"))
}

#[post(
    "/make_chart",
    format = "application/json",
    data = "<options>"
)]
pub async fn make_chart(options: String) {

    #[derive(serde::Deserialize, Debug)]
    struct Options {
        pub accounts: Vec<String>,
        pub markers: Option<u32>,
        pub start: Option<String>,
        pub end: Option<String>,
    }

    let options: Options = serde_json::from_str(&options).unwrap();

    println!("{:?}", &options);

    rocket::tokio::task::spawn_blocking(move || {
        plot::chart_from_options(
            options.accounts,
            options.start,
            options.end,
            options.markers
        ).unwrap();
    }).await.unwrap();
}

#[get("/accounts")]
pub async fn accounts() -> rocket::serde::json::Json<Vec<String>> {
    return rocket::serde::json::Json(
        read_history().await.unwrap().get_accounts()
    );
}







