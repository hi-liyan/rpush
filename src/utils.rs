//! # 小工具👸🏻

/// 判断字符串切片是否为空
pub fn is_empty(val: &str) -> bool {
    val.trim().len() == 0
}

/// 去掉字符串开始的路径分隔符
pub fn del_start_separator(path: &str) -> &str {
    if path.starts_with(std::path::MAIN_SEPARATOR) {
        return &path[1..];
    }
    path
}