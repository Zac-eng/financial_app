use actix_web::{get, HttpResponse, Responder};
use reqwest::{Client, Response};
use crate::structs::response::AssetForReturn;

#[get("/")]
pub async fn landing() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("
            <h1>This is a main page</h1>
            <h2>you can get asset list on /list</h2>
        ")
}

#[get("/current")]
async fn list() -> HttpResponse {
    let client = Client::new();
    let obj = get_info(&client).await;
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
    let asset_response = AssetForReturn {
        assets: resbody.text().await.unwrap()
    };
	HttpResponse::Ok().json(asset_response)
}

async fn get_info(client: &Client) -> String {
    let res = client.get("https://api.aevo.xyz/index?asset=BTC").send().await.expect("failed to get information form aevo");
    return parse_index(res).await
}

async fn parse_index(res: Response) -> String {
    res.text().await.expect("failed to parse response")
}
