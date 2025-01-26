use llm_demo::llmchain::tools::{
    bilibili::BilibiliClient, ip::IpClient, sqlite::SqliteTool, time::TimeTool,
};
use rusqlite::{Connection, Result};

async fn test_in_memory_db() -> Result<()> {
    // 创建内存数据库连接
    let conn = Connection::open_in_memory()?;

    // 创建测试表
    conn.execute_batch(
        r#"
        CREATE TABLE user (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT UNIQUE
        );
        
        CREATE TABLE order (
            order_id INTEGER PRIMARY KEY,
            user_id INTEGER REFERENCES user(id),
            amount REAL
        );
        "#,
    )?;

    // 初始化 SqliteTool
    let db_tool = SqliteTool::from_connection(conn);

    // 获取表结构
    let tables = db_tool.get_tables()?;

    // 打印结果
    println!("\n内存数据库表结构:");
    for (table_name, create_sql) in &tables {
        println!("[表名] {}\n[结构]\n{}\n", table_name, create_sql);
    }

    // 验证结果
    assert!(tables.contains_key("user"));
    assert!(tables.contains_key("order"));

    Ok(())
}

async fn test_file_db() -> Result<()> {
    // 创建/打开文件数据库
    let conn = Connection::open("test.db")?;

    // 创建测试表
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS product (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL CHECK(price > 0)
        );
        "#,
    )?;

    // 初始化 SqliteTool
    let db_tool = SqliteTool::from_connection(conn);

    // 获取表结构
    let tables = db_tool.get_tables()?;

    println!("\n文件数据库表结构:");
    for (table_name, create_sql) in &tables {
        println!("[表名] {}\n[结构]\n{}\n", table_name, create_sql);
    }

    Ok(())
}
// #[tokio::main]
fn main() {

        
    // 测试时间工具
    // let time_tool = TimeTool::new();

    // println!("本地时间: {}", time_tool.get_local_time("%Y-%m-%d %H:%M:%S"));
    // println!("UTC时间: {}", time_tool.get_utc_time("%Y-%m-%d %H:%M:%S"));
    // println!("Unix时间戳: {}", time_tool.get_unix_timestamp());
    // println!("RFC3339格式: {}", time_tool.get_rfc3339());

    // match time_tool.get_time_in_timezone("Asia/Tokyo", "%Y-%m-%d %H:%M:%S") {
    //     Ok(time) => println!("东京时间: {}", time),
    //     Err(e) => println!("获取东京时间失败: {}", e),
    // }

    // // 测试Bilibili客户端
    // let bili_client = match BilibiliClient::new() {
    //     Ok(client) => client,
    //     Err(e) => {
    //         eprintln!("Failed to create Bilibili client: {}", e);
    //         return;
    //     }
    // };

    // // 示例：搜索关键词
    // let keyword = "柯洁";
    // match bili_client.fetch_search(keyword).await {
    //     Ok(body) => {
    //         println!("Bilibili Response Body: {}", body);
    //     }
    //     Err(e) => println!("Bilibili Error: {}", e),
    // }

    // // 测试IP客户端
    // let ip_client = IpClient::new();

    // // 获取外网IP
    // match ip_client.get_external_ip().await {
    //     Ok(ip) => {
    //         println!("External IP: {}", ip);

    //         // 获取IP信息
    //         match ip_client.get_ip_info(&ip).await {
    //             Ok(info) => println!("IP Info: {}", info.text.ip2region),
    //             Err(e) => println!("Failed to get IP info: {}", e),
    //         }
    //     }
    //     Err(e) => println!("Failed to get external IP: {}", e),
    // }
}
