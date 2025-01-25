use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpInfoResponse {
    pub status: bool,
    pub message: String,
    pub text: IpInfoText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpInfoText {
    pub ip: String,
    pub l: i64,
    pub chunzhen: String,
    pub taobao: String,
    pub ipip: String,
    pub ip2region: String,
    pub geolite: String,
    pub dbip: String,
    pub ipDataCloud: String,
    pub amap: Option<String>,
}

pub fn decode_ip_info(json: &str) -> Result<IpInfoResponse, serde_json::Error> {
    serde_json::from_str(json)
}
