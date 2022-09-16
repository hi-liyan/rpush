use std::io::stdin;
use std::process;

use nu_ansi_term::Color::Green;

use crate::arg::get_matches;
use crate::config::{Config, ServerSpace};
use crate::utils as util;

mod config;
mod arg;
mod utils;

/// run method
pub fn run() {
    let matches = get_matches();

    if let Some(_) = matches.subcommand_matches("add") {
        handle_command_add();
    }

    if let Some(_) = matches.subcommand_matches("list") {
        handle_command_list();
    }
}

fn handle_command_add() {
    let mut name = String::new();
    let mut host = String::new();
    let mut path = String::new();
    let mut user = String::new();
    let mut pass = String::new();

    println!("{}", Green.paint("输入空间名称"));
    stdin().read_line(&mut name).expect("read_line error!");
    if util::is_empty(&name)  {
        eprintln!("空间名称不能为空！");
        process::exit(1);
    }

    println!("{}", Green.paint("输入主机地址"));
    stdin().read_line(&mut host).expect("read_line error!");
    if util::is_empty(&host)  {
        eprintln!("主机地址不能为空！");
        process::exit(1);
    }

    println!("{}", Green.paint("输入目标路径"));
    stdin().read_line(&mut path).expect("read_line error!");
    if util::is_empty(&path)  {
        eprintln!("目标路径不能为空！");
        process::exit(1);
    }

    println!("{}", Green.paint("输入主机用户名"));
    stdin().read_line(&mut user).expect("read_line error!");
    if util::is_empty(&user)  {
        eprintln!("主机用户名不能为空！");
        process::exit(1);
    }

    println!("{}", Green.paint("输入主机密码"));
    stdin().read_line(&mut pass).expect("read_line error!");
    if util::is_empty(&path)  {
        eprintln!("主机密码不能为空！");
        process::exit(1);
    }

    let server_space = ServerSpace::new(&name.trim(), &host.trim(),
                                 &path.trim(), &user.trim(), &pass.trim());
    Config::add_server_space(server_space);
    println!("🎉添加成功️");
}

fn handle_command_list() {
    let server_space_list = Config::list_server_space();
    for name in server_space_list {
        println!("➜ {}", Green.paint(name));
    }
}

