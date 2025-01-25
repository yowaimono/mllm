use futures::StreamExt;
use llm_demo::llmchain::stream::extract_content;
use llm_demo::llmchain::tools::ip::{fetch_ip_info, get_external_ip};
use llm_demo::llmchain::tools::ip_info::decode_ip_info;
use llm_demo::llmchain::tools::weather::get_weather;
use llm_demo::{DeepSeekLLM, Message, MessageRole};
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = llm_demo::DeepSeekConfig {
        api_key: "sk-a52d78227d9b49cbb4c10ac48db8fef4".to_string(),
        base_url: "https://api.deepseek.com/v1".to_string(),
        model: "deepseek-chat".to_string(),
    };
    let mut user_ip = String::new();

    match get_external_ip().await {
        Ok(ip) => {
            println!("你的外网 IP 地址是：{}", ip);
            user_ip = ip;
            
        }
        Err(e) => eprintln!("错误：{}", e),
    }

    // let ip = "106.6.148.161"; // 示例 IP 地址
    let result = fetch_ip_info(&user_ip).await?;
    println!("最终响应结果:\n{}", result);

    match decode_ip_info(&result) {
        Ok(info) => {
            println!("解析成功:");
            println!("IP: {}", info.text.ip);
            println!("Chunzhen: {}", info.text.chunzhen);
            println!("Taobao: {}", info.text.taobao);
            println!("IPIP: {}", info.text.ipip);
            println!("IP2Region: {}", info.text.ip2region);
            println!("GeoLite: {}", info.text.geolite);
            println!("DBIP: {}", info.text.dbip);
            println!("IPDataCloud: {}", info.text.ipDataCloud);
            println!("AMap: {:?}", info.text.amap);
        }
        Err(e) => {
            println!("解析失败: {}", e);
        }
    }

    // let city_code = "101240101"; // 示例城市代码
    // let city_name = "南昌"; // 示例城市名称

    // // 获取天气信息
    // let weather = match get_weather(city_code).await {
    //     Ok(w) => w,
    //     Err(e) => {
    //         eprintln!("获取天气失败: {}", e);
    //         return Ok(());
    //     }
    // };

    // // 获取当前时间
    // let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // // 初始化LLM
    // let llm = DeepSeekLLM::new(config);

    // // 构建提示词
    // let prompt = format!(
    //     "当前时间：{}\n\
    //     城市：{}\n\
    //     天气情况：{}\n\
    //     请根据以上信息生成一段天气播报词，带上一句电影台词和小表情",
    //     current_time, city_name, weather
    // );

    // // 发送给模型生成祝福语
    // let messages = vec![
    //     Message::new(
    //         MessageRole::System,
    //         "你是一位擅长创作诗意祝福的助手".to_string(),
    //     ),
    //     Message::new(MessageRole::User, prompt),
    // ];

    // match llm.invoke(messages).await {
    //     Ok(response) => println!("{}", response),
    //     Err(e) => eprintln!("生成祝福语失败: {}", e),
    // }
    // let llm = DeepSeekLLM::new(config);

    // let messages = vec![
    //     Message::new(
    //         MessageRole::System,
    //         "You are a helpful assistant.".to_string(),
    //     ),
    //     Message::new(MessageRole::User, "写个故事吧。".to_string()),
    // ];

    // Normal invocation
    // let response = llm.invoke(messages.clone()).await?;
    // println!("Response: {}", response);

    // Streaming response
    // let mut stream = llm.stream(messages).await?;
    // while let Some(result) = stream.next().await {
    //     match result {
    //         Ok(chunk) => {
    //             print!("{}",
    //                 match extract_content(chunk) {
    //                     Some(content) => content,
    //                     None => "No content found".to_string(),
    //                 }
    //             );
    //         }
    //         Err(e) => eprintln!("Error: {}", e),
    //     }
    // }

    Ok(())
}
