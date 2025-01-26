use reqwest::header::{
    self, HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, ORIGIN, REFERER, USER_AGENT,
};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

// 响应数据结构
#[derive(Debug, Deserialize)]
pub struct BilibiliResponse {
    pub code: i32,
    pub message: String,
    pub ttl: i32,
    pub data: BilibiliData,
}

#[derive(Debug, Deserialize)]
pub struct BilibiliData {
    pub list: Vec<Video>,
    pub no_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct Video {
    pub aid: i64,
    pub videos: i32,
    pub tid: i32,
    pub tname: String,
    pub copyright: i32,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: String,
    pub state: i32,
    pub duration: i32,
    pub mission_id: Option<i64>, // 可能为 null 或缺失
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub dynamic: String,
    pub cid: i64,
    pub dimension: Dimension,
    pub short_link_v2: String,
    pub first_frame: Option<String>, // 可能为 null 或缺失
    pub pub_location: String,
    pub cover43: String,
    pub tidv2: i32,
    pub tnamev2: String,
    pub bvid: String,
    pub season_type: i32,
    pub is_ogv: bool,
    pub ogv_info: Option<OgvInfo>, // 可能为 null 或缺失
    pub enable_vt: i32,
    pub ai_rcmd: Option<AiRcmd>, // 可能为 null 或缺失
    pub rcmd_reason: RcmdReason,
}

#[derive(Debug, Deserialize)]
pub struct Rights {
    pub bp: i32,
    pub elec: i32,
    pub download: i32,
    pub movie: i32,
    pub pay: i32,
    pub hd5: i32,
    pub no_reprint: i32,
    pub autoplay: i32,
    pub ugc_pay: i32,
    pub is_cooperation: i32,
    pub ugc_pay_preview: i32,
    pub no_background: i32,
    pub arc_pay: i32,
    pub pay_free_watch: i32,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub mid: i64,
    pub name: String,
    pub face: String,
}

#[derive(Debug, Deserialize)]
pub struct Stat {
    pub aid: i64,
    pub view: i64,
    pub danmaku: i64,
    pub reply: i64,
    pub favorite: i64,
    pub coin: i64,
    pub share: i64,
    pub now_rank: i32,
    pub his_rank: i32,
    pub like: i64,
    pub dislike: i32,
    pub vt: i32,
    pub vv: i64,
}

#[derive(Debug, Deserialize)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
    pub rotate: i32,
}

#[derive(Debug, Deserialize)]
pub struct OgvInfo {
    // 根据实际数据结构补充字段
}

#[derive(Debug, Deserialize)]
pub struct AiRcmd {
    // 根据实际数据结构补充字段
}

#[derive(Debug, Deserialize)]
pub struct RcmdReason {
    pub content: String,
    pub corner_mark: i32,
}

// Bilibili客户端结构体
pub struct BilibiliClient {
    client: Client,
    headers: HeaderMap,
}

// 解析函数
pub fn parse_bilibili_response(response_text: &str) -> Result<BilibiliResponse, Box<dyn Error>> {
    let json_data: BilibiliResponse = serde_json::from_str(response_text)?;
    Ok(json_data)
}

impl BilibiliClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let mut headers = HeaderMap::new();
        headers.insert(header::HOST, "api.bilibili.com".parse()?);
        headers.insert(ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9".parse()?);
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36".parse()?,
        );
        headers.insert(header::COOKIE, "".parse()?);
        headers.insert(ACCEPT, "*/*".parse()?);
        headers.insert(ORIGIN, "https://www.bilibili.com".parse()?);
        headers.insert(REFERER, "https://www.bilibili.com/v/popular/all/".parse()?);
        headers.insert(header::ACCEPT_ENCODING, "gzip, deflate, br".parse()?);

        Ok(Self { client, headers })
    }

    pub async fn fetch_popular(&self) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get("https://api.bilibili.com/x/web-interface/popular?ps=20&pn=1")
            .headers(self.headers.clone())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(format!("Request failed with status code: {}", response.status()).into())
        }
    }

    pub async fn fetch_ranking(&self) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get("https://api.bilibili.com/x/web-interface/ranking/v2")
            .headers(self.headers.clone())
            .send()
            .await?;

        Ok(response.text().await?)
    }

    pub async fn fetch_search(&self, keyword: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://api.bilibili.com/x/web-interface/wbi/search/all/v2?keyword={}&4",
            keyword
        );

        let response = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .send()
            .await?;

        Ok(response.text().await?)
    }
}

// use reqwest::header;
// use serde::Deserialize;
// use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct BilibiliRankingResponse {
    pub code: i32,
    pub message: String,
    pub ttl: i32,
    pub data: BilibiliRankingData,
}

#[derive(Debug, Deserialize)]
pub struct BilibiliRankingData {
    pub list: Vec<RankingVideo>,
}

#[derive(Debug, Deserialize)]
pub struct RankingVideo {
    pub aid: i64,
    pub videos: i32,
    pub tid: i32,
    pub tname: String,
    pub copyright: i32,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: String, // 可能为 null
    pub state: i32,
    pub duration: i32,
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub dynamic: Option<String>, // 可能为 null
    pub cid: i64,
    pub dimension: Dimension,
    pub short_link_v2: String,
    pub first_frame: Option<String>,  // 可能为 null
    pub pub_location: Option<String>, // 可能为 null
    pub cover43: Option<String>,      // 可能为 null
    pub tidv2: i32,
    pub tnamev2: String,
    pub bvid: String,
    pub score: i32,
    pub enable_vt: i32,
}

// #[derive(Debug, Deserialize)]
// pub struct Rights {
//     pub bp: i32,
//     pub elec: i32,
//     pub download: i32,
//     pub movie: i32,
//     pub pay: i32,
//     pub hd5: i32,
//     pub no_reprint: i32,
//     pub autoplay: i32,
//     pub ugc_pay: i32,
//     pub is_cooperation: i32,
//     pub ugc_pay_preview: i32,
//     pub no_background: i32,
//     pub arc_pay: i32,
//     pub pay_free_watch: i32,
// }

// #[derive(Debug, Deserialize)]
// pub struct Owner {
//     pub mid: i64,
//     pub name: String,
//     pub face: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct Stat {
//     pub aid: i64,
//     pub view: i64,
//     pub danmaku: i64,
//     pub reply: i64,
//     pub favorite: i64,
//     pub coin: i64,
//     pub share: i64,
//     pub now_rank: i32,
//     pub his_rank: i32,
//     pub like: i64,
//     pub dislike: i32,
//     pub vt: i32,
//     pub vv: i64,
// }

// #[derive(Debug, Deserialize)]
// pub struct Dimension {
//     pub width: i32,
//     pub height: i32,
//     pub rotate: i32,
// }

// 解析函数
pub fn parse_bilibili_ranking_response(
    response_text: &str,
) -> Result<BilibiliRankingResponse, Box<dyn Error>> {
    let json_data: BilibiliRankingResponse = serde_json::from_str(response_text)?;
    Ok(json_data)
}
