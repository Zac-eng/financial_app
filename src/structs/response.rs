use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct AssetForReturn {
	pub assets: String
}

#[derive(Serialize, Deserialize)]
pub struct HistoryResponse {
	pub asset: String,
	pub history: Vec<[String;2]>,
}

#[derive(Deserialize)]
pub struct HistoryFromAevo {
	pub history: Vec<[String;2]>,
}

// #[derive(Deserialize)]
// pub struct AssetFromServer {
// }

// pub trait Response{
	
// }

// pub trait ArrayResponse is Response {

// }