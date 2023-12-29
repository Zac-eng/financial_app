use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HistoryResponse {
	pub asset: String,
	pub history: Vec<[String;2]>,
}

#[derive(Deserialize)]
pub struct HistoryFromAevo {
	pub history: Vec<[String;2]>,
}
