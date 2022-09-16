use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSpace {
    pub name: String,
    pub host: String,
    pub path: String,
    pub user: String,
    pub pass: String,
}

impl ServerSpace {
    pub fn new(name: &str, host: &str, path: &str, user: &str, pass: &str) -> Self {
        Self {
            name: String::from(name),
            host: String::from(host),
            path: String::from(path),
            user: String::from(user),
            pass: String::from(pass),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    server_space_list: HashMap<String, ServerSpace>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_space_list: HashMap::new()
        }
    }
}

impl Config {
    pub fn add_server_space(server_space: ServerSpace) {
        let mut cfg = get_config();
        let mut spaces = cfg.server_space_list;

        match spaces.get(&server_space.name) {
            Some(_) => println!("空间已存在！"),
            None => {
                spaces.insert(server_space.name.clone(), server_space);
                cfg.server_space_list = spaces;
                save_config(cfg);
            }
        }
    }

    pub fn list_server_space() -> Vec<String> {
        let cfg = get_config();
        let spaces = cfg.server_space_list;
        spaces.values()
            .into_iter()
            .map(|server_space| server_space.name.clone())
            .collect::<Vec<String>>()
    }
}

#[test]
fn test_add_server_space() {
    let space = ServerSpace::new("aaa", "bbb", "ccc", "ddd", "eee");
    Config::add_server_space(space);
}

#[test]
fn test_list_server_space() {
    let list = Config::list_server_space();
    println!("{:?}", list);
}

fn get_config_path() -> String {
    let home_dir = env::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/rpush_config"
}

fn get_config() -> Config {
    let cfg: Config = confy::load_path(get_config_path()).expect("load config file error!");
    cfg
}

fn save_config(cfg: Config) {
    confy::store_path(get_config_path(), cfg).expect("save config file error!");
}
