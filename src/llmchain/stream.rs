pub fn extract_content(chunk: String) -> Option<String> {
    // 去掉开头的 "data: " 和结尾的换行符
    let json_str = chunk.trim_start_matches("data: ").trim();

    // 找到 "content" 字段的位置
    if let Some(content_start) = json_str.find(r#""content":""#) {
        let content_start = content_start + r#""content":""#.len();
        let content_end = json_str[content_start..].find('"').unwrap_or(0) + content_start;

        // 提取 content 的值
        let content = json_str[content_start..content_end].to_string();
        Some(content)
    } else {
        None
    }
}
