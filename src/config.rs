//! server configuration
//! add/list/remove/detail server configuration.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use nu_ansi_term::Color::Green;

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

impl Display for ServerSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "空间名称：{}\n主机地址：{}\n目标路径：{}\n用户名：{}\n密码：{}",
               Green.paint(&self.name), Green.paint(&self.host), Green.paint(&self.path),
        Green.paint(&self.user), Green.paint(&self.pass))
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
        let server_space_list = &mut cfg.server_space_list;

        match server_space_list.get(&server_space.name) {
            Some(_) => println!("空间已存在！"),
            None => {
                server_space_list.insert(server_space.name.clone(), server_space);
                save_config(cfg);
            }
        }
    }

    pub fn list_server_space() -> Vec<String> {
        let cfg = get_config();
        let server_space_list = &cfg.server_space_list;
        server_space_list.values()
            .into_iter()
            .map(|server_space| server_space.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn server_space_detail(server_space_name: &str) -> Option<ServerSpace> {
        let cfg = get_config();
        let server_space_list =  &cfg.server_space_list;
        let server_space_opt = server_space_list.get(server_space_name);
        match server_space_opt {
            Some(server_space) => {
                Some(ServerSpace {
                    name: server_space.name.clone(),
                    host: server_space.host.clone(),
                    path: server_space.path.clone(),
                    user: server_space.name.clone(),
                    pass: server_space.pass.clone()
                })
            },
            None => None
        }
    }

    pub fn remove_server_space(server_space_name: &str) -> bool {
        let mut cfg = get_config();
        let server_space_list = &mut cfg.server_space_list;
        let server_space_option = server_space_list.get(server_space_name);
        match server_space_option {
            Some(_) => {
                server_space_list.remove(server_space_name);
                save_config(cfg);
                true
            },
            None => false
        }
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

#[test]
fn test_server_space_detail() {
    let server_space = Config::server_space_detail("test2");
    println!("{:?}", server_space);
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
