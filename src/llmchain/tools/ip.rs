use regex::Regex;
use reqwest::{Client, Response};
use std::error::Error;

use serde::{Deserialize, Serialize};
pub struct IpClient {
    client: Client,
}

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

impl IpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn decode_ip_info(&self, json: &str) -> Result<IpInfoResponse, serde_json::Error> {
        serde_json::from_str(json)
    }
    pub async fn get_external_ip(&self) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .get("https://tool.lu/ip/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("请求失败，状态码: {}", response.status()).into());
        }

        let body = response.text().await?;
        let re = Regex::new(r"<p>你的外网IP地址是：(\d+\.\d+\.\d+\.\d+)</p>")?;
        if let Some(caps) = re.captures(&body) {
            if let Some(ip) = caps.get(1) {
                return Ok(ip.as_str().to_string());
            }
        }

        Err("未找到 IP 地址".into())
    }

    pub async fn get_ip_info(&self, ip: &str) -> Result<IpInfoResponse, Box<dyn Error>> {
        let json = self.fetch_ip_info(ip).await?;
        Ok(self.decode_ip_info(&json).await?)
    }

    pub async fn fetch_ip_info(&self, ip: &str) -> Result<String, Box<dyn Error>> {
        let (client, session) = self.get_session_cookie().await?;
        let (client, (uuid, access)) = self.get_uuid_and_access(client).await?;
        let response = self
            .send_post_request(client, &session, &uuid, &access, ip)
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(format!("POST 请求失败: {} - {}", status, body).into());
        }

        response.text().await.map_err(|e| e.into())
    }

    async fn get_session_cookie(&self) -> Result<(Client, String), Box<dyn Error>> {
        let response = self.client
            .get("https://tool.lu/ip/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
            .send()
            .await?;

        self.extract_cookie(&response, "_session")
            .and_then(|s| Ok((self.client.clone(), s)))
    }

    async fn get_uuid_and_access(
        &self,
        client: Client,
    ) -> Result<(Client, (String, String)), Box<dyn Error>> {
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

        let uuid = self.extract_cookie(&response, "uuid")?;
        let access = self.extract_cookie(&response, "_access")?;
        Ok((client, (uuid, access)))
    }

    async fn send_post_request(
        &self,
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

    fn extract_cookie(&self, response: &Response, target: &str) -> Result<String, Box<dyn Error>> {
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
}
