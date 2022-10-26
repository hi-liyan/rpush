//! # 小工具👸🏻

use std::io::stdin;

/// 判断字符串切片是否为空
pub fn is_empty(val: &str) -> bool {
    val.trim().len() == 0
}

/// 去掉路径开始的路径分隔符
pub fn del_start_separator(path: &str) -> &str {
    if path.starts_with(std::path::MAIN_SEPARATOR) {
        return &path[1..];
    }
    path
}

/// 读取控制台输入
pub fn read_console() -> String {
    let mut v: String = String::new();
    stdin().read_line(&mut v).expect("read_line error!");
    String::from(v.trim())
}