#[derive(serde::Deserialize)]
pub struct AssetInfo {
    pub asset: String,
}

#[derive(serde::Deserialize)]
pub struct HistoryRequest {
	pub asset: String,
	pub resolution: Option<u8>,
	pub start_time: Option<u32>,
	pub end_time: Option<u32>,
}

// #[derive(serde::Serialize)]
// enum Resolution {
// 	Xs=30,
// 	Sm=60,
// 	Md=90,
// 	Lg=120
// }
