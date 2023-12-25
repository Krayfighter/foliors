
#![feature(slice_partition_dedup)]

#[macro_use] extern crate rocket;

use rocket::fs::relative;

use rocket_dyn_templates::{Template, context};

pub mod util;
use util::*;

pub mod holdings;
use holdings::*;

mod dataroutes;
use dataroutes::*;


const INDEX_PAGE: &str = "static/pages/index.html";

#[get("/")]
async fn index() -> rocket::fs::NamedFile {
    return rocket::fs::NamedFile::open(INDEX_PAGE)
        .await
        .unwrap();
}

#[get("/view")]
async fn view() -> Template {
    let holdings = read_holdings().await.unwrap();

    return Template::render("view_holdings", context! { holdings: holdings });
}

#[get("/holdings")]
async fn view_holdings() -> Template {

    let asset_list = read_holdings()
        .await
        .unwrap()
        .enumerate_assets();

    return Template::render(
        "enter_asset_values", context!{ assets: asset_list }
    );
}

#[get("/view_history")]
async fn view_history() -> rocket::fs::NamedFile {
    return rocket::fs::NamedFile::open("static/pages/history.html").await.unwrap();
}

#[get("/chart")]
async fn chart() -> rocket::fs::NamedFile {
    return rocket::fs::NamedFile::open(
        "static/pages/chart.html"
    ).await.unwrap();
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    
    let _ = rocket::build()
        .attach(Template::fairing())
        .mount("/static", rocket::fs::FileServer::from(relative!("static")))
        .mount("/", routes![
            index,
            view,
            view_holdings,
            view_history,
            chart,
            dataroutes::get_history_json,
            dataroutes::register_prices,
            dataroutes::make_chart,
            dataroutes::accounts,
        ])
        .launch()
        .await
    ;

    return Ok(());

}

