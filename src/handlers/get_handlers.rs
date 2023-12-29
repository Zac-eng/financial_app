use actix_web::{web, get, HttpResponse, Responder};
use reqwest::Client;
//use plotters::prelude::*;
use crate::structs::response::{HistoryResponse, HistoryFromAevo};
use crate::structs::request::{AssetInfo, HistoryRequest};
use crate::structs::aevoinfo::AevoInfo;
use std::env;
use dotenv::dotenv;

#[get("/")]
pub async fn landing() -> HttpResponse {
    dotenv().ok();

    let aevo_path= env::var("AEVO_PATH").expect("failed to road AEVO_PATH");
    let client = Client::new();
    let res = client.get(aevo_path).send().await.unwrap();
    let res_body: AevoInfo = serde_json::from_str(&res.text().await.unwrap()).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("
            <h1>This is a main page on {}</h1>
            <h2>you can get asset list on /list</h2>
            <h3>chain id: {}</h3>
        ", res_body.domain.name, res_body.domain.chainId))
}

#[get("/current")]
async fn list(query: web::Query<AssetInfo>) -> impl Responder {
    let client = Client::new();
    let res = client.get(format!("https://api.aevo.xyz/index?asset={}", query.asset)).send().await.unwrap();
    let obj = res.text().await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(obj)
}

#[get("/assets")]
pub async fn get_assets() -> impl Responder {
	let client = Client::new();
	let obj = client.get("https://api.aevo.xyz/assets").send().await;
    let resbody = match obj {
        Ok(object) => {
            object
        }
        Err(_) => {
            return HttpResponse::InternalServerError().content_type("text/html").body("<h1>500 Internal server error<h1/>")
        }
    };
    let asset_response: Vec<String> = serde_json::from_str(&resbody.text().await.unwrap()).unwrap();
	HttpResponse::Ok().json(asset_response)
}

#[get("/history")]
pub async fn get_history(query: web::Query<HistoryRequest>) -> impl Responder {
    let client = Client::new();
    let mut endpoint = format!("https://api.aevo.xyz/index-history?asset={}", query.asset);
    if query.resolution.is_some() {endpoint += &format!("&resolution={}", query.resolution.unwrap());}
    if query.start_time.is_some() {endpoint += &format!("&start_time={}", query.start_time.unwrap());}
    if query.end_time.is_some() {endpoint += &format!("&end_time={}", query.end_time.unwrap());}
    let res = client.get(endpoint).send().await.expect("failed to send");
    let obj = HistoryResponse {
        asset: query.asset.clone(),
        history: res.json::<HistoryFromAevo>().await.expect("failed to deserialize").history,
    };
    println!("{:?}", obj.history);
    //put_history(&query.asset, &obj.history);
    HttpResponse::Ok().json(obj)
}

// fn put_history(asset: &str, history: &Vec<[String;2]>) {
//     let image_width = 1080;
//     let image_height = 720;

//     let mut ys: Vec<f64> = Vec::new();
//     let mut xs: Vec<f64> = Vec::new();
//     for price in history {
//         xs.push(price[0].parse().unwrap());
//         ys.push(price[1].parse().unwrap());
//     }
//     let path = format!("{}_movement.png", asset);
//     let root = BitMapBackend::new
//     (&path, (image_width, image_height)).into_drawing_area();
//     root.fill(&WHITE).unwrap();
//     let (y_min, y_max) = ys.iter()
//     .fold(
//       (0.0/0.0, 0.0/0.0),
//       |(m,n), v| (v.min(m), v.max(n))
//      );

//     let caption = "Sample Plot";
//     let font = ("sans-serif", 20);

//     let mut chart = ChartBuilder::on(&root)
//         .caption(caption, font.into_font())
//         .margin(10)
//         .x_label_area_size(16)
//         .y_label_area_size(42)
//         .build_cartesian_2d(
// *xs.first().unwrap()..*xs.last().unwrap(),
//                     y_min..y_max
//     ).unwrap();
//     chart.configure_mesh().draw().unwrap();
//     let line_series = LineSeries::new(
//         xs.iter()
//           .zip(ys.iter())
//           .map(|(x, y)| (*x, *y)),
//         &RED
//        );
//     chart.draw_series(line_series).unwrap();
// }
