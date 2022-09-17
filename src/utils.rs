//! 小工具👸🏻

pub fn is_empty(val: &str) -> bool {
    val.trim().len() == 0
}

pub fn del_start_separator(path: &str) -> &str {
    if path.starts_with(std::path::MAIN_SEPARATOR) {
        return &path[1..];
    }
    path
}