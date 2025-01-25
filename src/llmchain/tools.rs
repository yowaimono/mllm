use chrono::prelude::*;
use regex::Regex;
use reqwest::header;
use reqwest::Client;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn get_weather(city_code: &str) -> Result<String, Box<dyn Error>> {
    if city_code.is_empty() {
        return Err("错误：请提供城市代码。".into());
    }

    // 构造 URL
    let timestamp = Utc::now().timestamp_millis();
    let url = format!(
        "https://d1.weather.com.cn/dingzhi/{}.html?_{}",
        city_code, timestamp
    );

    // 构造 Cookie
    let cookie = format!(
        "Hm_lvt_080dabacb001ad3dc8b9b9049b36d43b=1737803204; Hm_lpvt_080dabacb001ad3dc8b9b9049b36d43b=1737803204; HMACCOUNT=293F62AF50E6FA79; f_city=%E5%8D%97%E6%98%8C%7C{}%7C",
        city_code
    );

    // 创建 HTTP 客户端
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // 忽略 SSL 证书验证
        .build()?;

    // 发送请求
    let response = client
        .get(&url)
        .header(header::HOST, "d1.weather.com.cn")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
        .header(header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9")
        .header(header::ACCEPT, "*/*")
        .header(header::REFERER, "https://www.weather.com.cn/")
        .header(header::CONNECTION, "keep-alive")
        .header(header::COOKIE, cookie)
        .send()
        .await?;

    // 检查响应状态
    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(format!("请求失败，状态码: {}", response.status()).into())
    }
}

pub async fn get_external_ip() -> Result<String, Box<dyn Error>> {
    // 创建 HTTP 客户端
    let client = Client::new();

    // 发送 GET 请求
    let response = client
        .get("https://tool.lu/ip/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
        .send()
        .await?;

    // 检查响应状态
    if !response.status().is_success() {
        return Err(format!("请求失败，状态码: {}", response.status()).into());
    }

    // 读取响应内容
    let body = response.text().await?;

    // 使用正则表达式匹配 IP 地址
    let re = Regex::new(r"<p>你的外网IP地址是：(\d+\.\d+\.\d+\.\d+)</p>")?;
    if let Some(caps) = re.captures(&body) {
        if let Some(ip) = caps.get(1) {
            return Ok(ip.as_str().to_string());
        }
    }

    Err("未找到 IP 地址".into())
}

// use reqwest::{Client, Response};
// use std::error::Error;

/// 公共方法：获取 IP 信息
pub async fn fetch_ip_info(ip: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    // 第一步：获取 _session cookie
    let (client, session) = get_session_cookie(client).await?;

    // 第二步：获取 uuid 和 _access cookies
    let (client, (uuid, access)) = get_uuid_and_access(client).await?;

    // 第三步：携带所有 cookies 发送 POST 请求
    let response = send_post_request(client, &session, &uuid, &access, ip).await?;

    // 检查响应状态
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        return Err(format!("POST 请求失败: {} - {}", status, body).into());
    }

    // 返回响应内容
    response.text().await.map_err(|e| e.into())
}

// 获取 _session cookie
async fn get_session_cookie(client: Client) -> Result<(Client, String), Box<dyn Error>> {
    let response = client
        .get("https://tool.lu/ip/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
        .send()
        .await?;

    extract_cookie(&response, "_session").and_then(|s| Ok((client, s)))
}

// 获取 uuid 和 _access cookies
async fn get_uuid_and_access(client: Client) -> Result<(Client, (String, String)), Box<dyn Error>> {
    let response = client
        .get("https://a.tool.lu/te.js")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
        .header("Origin", "https://tool.lu")
        .header("Referer", "https://tool.lu/")
        .header("Sec-Ch-Ua", "\"Not?A_Brand\";v=\"99\", \"Chromium\";v=\"130\"")
        .header("Sec-Ch-Ua-Mobile", "?0")
        .header("Sec-Ch-Ua-Platform", "\"Windows\"")
        .header("Accept-Language", "zh-CN,zh;q=0.9")
        .header("Sec-Fetch-Site", "same-site")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Dest", "script")
        .send()
        .await?;

    let uuid = extract_cookie(&response, "uuid")?;
    let access = extract_cookie(&response, "_access")?;
    Ok((client, (uuid, access)))
}

// 发送最终 POST 请求
async fn send_post_request(
    client: Client,
    session: &str,
    uuid: &str,
    access: &str,
    ip: &str,
) -> Result<Response, Box<dyn Error>> {
    let cookie = format!("_session={}; uuid={}; _access={}", session, uuid, access);

    client
        .post("https://tool.lu/ip/ajax.html")
        .header("Cookie", cookie)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .header("Origin", "https://tool.lu")
        .header("Referer", "https://tool.lu/ip/")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Dest", "empty")
        .body(format!("ip={}", ip))
        .send()
        .await
        .map_err(|e| e.into())
}

// 通用 cookie 提取函数
fn extract_cookie(response: &Response, target: &str) -> Result<String, Box<dyn Error>> {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .filter_map(|header| header.to_str().ok())
        .flat_map(|header| header.split(','))
        .filter_map(|cookie| {
            cookie
                .split(';')
                .next()?
                .split_once('=')
                .filter(|(k, _)| k.trim() == target)
        })
        .map(|(_, v)| v.trim().to_string())
        .next()
        .ok_or_else(|| format!("未找到 {} cookie", target).into())
}

// use serde::{Deserialize, Serialize};
// use serde_json::Result;

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
