use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AevoInfo {
    pub domain: Domain,
    pub l2ChainId: String,
    pub contracts: Contracts,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub name: String,
    pub version: String,
    pub chainId: String,
}

#[derive(Debug, Deserialize)]
pub struct Contracts {
    pub withdrawProxy: String,
    pub collateral: Collateral,
}

#[derive(Debug, Deserialize)]
pub struct Collateral {
    pub USDC: String,
    pub USDT: String,
    pub WETH: String,
}
