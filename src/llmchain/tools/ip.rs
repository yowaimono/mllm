use regex::Regex;
use reqwest::{Client, Response};
use std::error::Error;

pub async fn get_external_ip() -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let response = client
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

pub async fn fetch_ip_info(ip: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let (client, session) = get_session_cookie(client).await?;
    let (client, (uuid, access)) = get_uuid_and_access(client).await?;
    let response = send_post_request(client, &session, &uuid, &access, ip).await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        return Err(format!("POST 请求失败: {} - {}", status, body).into());
    }

    response.text().await.map_err(|e| e.into())
}

async fn get_session_cookie(client: Client) -> Result<(Client, String), Box<dyn Error>> {
    let response = client
        .get("https://tool.lu/ip/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6723.70 Safari/537.36")
        .send()
        .await?;

    extract_cookie(&response, "_session").and_then(|s| Ok((client, s)))
}

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
