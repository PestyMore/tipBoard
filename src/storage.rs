use std::fs;
use std::path::PathBuf;
use crate::types::Part;

pub fn get_data_path() -> PathBuf {
    // 开发模式下保存在当前目录，打包模式下保存在 exe 所在目录(实现真正的便携化)
    if cfg!(debug_assertions) {
        PathBuf::from("data.json")
    } else {
        if let Ok(mut path) = std::env::current_exe() {
            path.pop(); // 移除 exe 文件名，保留目录
            path.push("data.json");
            path
        } else {
            PathBuf::from("data.json")
        }
    }
}

pub fn load_data() -> Vec<Part> {
    if let Ok(content) = fs::read_to_string(get_data_path()) {
        if let Ok(parsed) = serde_json::from_str::<Vec<Part>>(&content) {
            return parsed;
        }
    }
    Vec::new()
}

pub fn save_data(data: &Vec<Part>) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = fs::write(get_data_path(), json);
    }
}
