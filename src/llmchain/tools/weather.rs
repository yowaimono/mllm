use chrono::prelude::*;
use reqwest::header;
use reqwest::Client;
use std::error::Error;

pub async fn get_weather(city_code: &str) -> Result<String, Box<dyn Error>> {
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

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

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

    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(format!("请求失败，状态码: {}", response.status()).into())
    }
}
