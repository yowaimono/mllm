use chrono::prelude::*;
use reqwest::header::{self, HeaderMap};
use reqwest::Client;
use std::error::Error;

pub struct WeatherClient {
    client: Client,
    headers: HeaderMap,
}

impl WeatherClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let mut headers = HeaderMap::new();
        headers.insert(header::HOST, "d1.weather.com.cn".parse()?);
        headers.insert(
            header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36".parse()?,
        );
        headers.insert(header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9".parse()?);
        headers.insert(header::ACCEPT, "*/*".parse()?);
        headers.insert(header::REFERER, "https://www.weather.com.cn/".parse()?);
        headers.insert(header::CONNECTION, "keep-alive".parse()?);

        Ok(Self { client, headers })
    }

    pub async fn get_weather(&self, city_code: &str) -> Result<String, Box<dyn Error>> {
        if city_code.is_empty() {
            return Err("错误：请提供城市代码。".into());
        }

        let timestamp = Utc::now().timestamp_millis();
        let url = format!(
            "https://d1.weather.com.cn/dingzhi/{}.html?_{}",
            city_code, timestamp
        );

        let cookie = format!(
            "Hm_lvt_080dabacb001ad3dc8b9b9049b36d43b=1737803204; Hm_lpvt_080dabacb001ad3dc8b9b9049b36d43b=1737803204; HMACCOUNT=293F62AF50E6FA79; f_city=%E5%8D%97%E6%98%8C%7C{}%7C",
            city_code
        );

        let mut request_headers = self.headers.clone();
        request_headers.insert(header::COOKIE, cookie.parse()?);

        let response = self.client
            .get(&url)
            .headers(request_headers)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(format!("请求失败，状态码: {}", response.status()).into())
        }
    }
}
