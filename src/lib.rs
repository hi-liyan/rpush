//! # rpush✨
//!
//! ## 介绍
//!
//! 一个推送本地文件到服务器空间的小工具。
//!
//! 工具可以保存多个服务器空间配置信息（主机地址、目标路径、用户名、密码），配置文件保存在当前用户目录，文件名：`.rpush_config` 。
//!
//! ## 用法
//!
//! 1. 添加服务器配置
//! ```bash
//! rpush add
//! ```
//!
//! 2. 列出已添加的服务器配置
//! ```bash
//! rpush list
//! ```
//!
//! 3. 查看服务器配置详情
//! ```bash
//! rpush detail <space_name>
//! ```
//!
//! 4. 移除服务器配置
//! ```bash
//! rpush remove <space_name>
//! ```
//!
//! 5. 将当前目录下的指定目录推送到指定服务器。这里要注意，<pushed_dir> 指的是当前目录下要推送的目录，推送到空间中的是该目录中的所有内容。
//! ```bash
//! rpush push <pushed_dir> <space_name>
//! ```
//!
//! 6. 删除服务器空间中的所有文件（使用的 rm -rf 命令）
//! ```bash
//! rpush rmrf <space_name>
//! ```

#[macro_use]
extern crate clap;
extern crate base64;

use std::{
    cmp::Ordering,
    env,
    error::Error,
    fs::{self, File},
    io::stdin,
    path::{Path, PathBuf},
    sync::Arc,
};

use std::net::TcpStream;
use clap::ArgMatches;

use flate2::{
    Compression,
    write::GzEncoder,
};
use indicatif::ProgressBar;
use nu_ansi_term::Color::{Green, Red};
use ssh_rs::{ssh, error::SshError, LocalSession};

use crate::arg::get_matches;
use crate::config::{Config, ServerSpace};
use crate::utils as util;
use crate::aes::{encrypt, decrypt};
use crate::msg::{
    ADD_SUCCESS,
    HOST_ADDRESS_IS_EMPTY,
    INPUT_HOST_ADDRESS,
    INPUT_PASSWORD,
    INPUT_SPACE_NAME_MSG,
    INPUT_TARGET_PATH,
    INPUT_USERNAME,
    IS_NOT_DIR,
    PASSWORD_IS_EMPTY,
    REMOVE_SUCCESS,
    RMRF_CONFIRM,
    RMRF_SUCCESS,
    SPACE_LIST_IS_EMPTY,
    SPACE_LIST_TITLE,
    SPACE_NAME_IS_EMPTY,
    SPACE_NAME_IS_EXISTED,
    SPACE_NAME_IS_NOT_EXISTED,
    TARGET_PATH_IS_EMPTY,
    UPLOAD_ERR,
    UPLOAD_SUCCESS,
    USERNAME_IS_EMPTY
};
use crate::util::read_console;

mod config;
mod arg;
mod utils;
mod aes;
mod msg;

/// run func
pub fn run() {
    let arg_matches = get_matches();
    if let Some(_) = arg_matches.subcommand_matches("add") {
        handle_command_add();
    }
    if let Some(_) = arg_matches.subcommand_matches("list") {
        handle_command_list();
    }
    if let Some(arg_matches) = arg_matches.subcommand_matches("detail") {
        handle_command_detail(arg_matches);
    }
    if let Some(arg_matches) = arg_matches.subcommand_matches("remove") {
        handle_command_remove(arg_matches);
    }
    if let Some(arg_matches) = arg_matches.subcommand_matches("push") {
        handle_command_push(arg_matches);
    }
    if let Some(arg_matches) = arg_matches.subcommand_matches("rmrf") {
        handle_command_rmrf(arg_matches);
    }
}

fn handle_command_add() {
    println!("{}", Green.paint(INPUT_SPACE_NAME_MSG));
    let name = read_console();
    if util::is_empty(&name) {
        eprintln!("{}", SPACE_NAME_IS_EMPTY);
        return;
    }
    if !Config::check_server_space_name_available(&name) {
        eprintln!("{}", SPACE_NAME_IS_EXISTED);
        return;
    }

    println!("{}", Green.paint(INPUT_HOST_ADDRESS));
    let host = read_console();
    if util::is_empty(&host) {
        eprintln!("{}", HOST_ADDRESS_IS_EMPTY);
        return;
    }

    println!("{}", Green.paint(INPUT_TARGET_PATH));
    let path = read_console();
    if util::is_empty(&path) {
        eprintln!("{}", TARGET_PATH_IS_EMPTY);
        return;
    }

    println!("{}", Green.paint(INPUT_USERNAME));
    let user = read_console();
    if util::is_empty(&user) {
        eprintln!("{}", USERNAME_IS_EMPTY);
        return;
    }

    println!("{}", Green.paint(INPUT_PASSWORD));
    let pass = rpassword::read_password().unwrap();
    if util::is_empty(&pass.trim()) {
        eprintln!("{}", PASSWORD_IS_EMPTY);
        return;
    }
    let pass = encrypt(&pass).unwrap();
    let server_space = ServerSpace::new(&name, &host, &path, &user, &pass);
    match Config::add_server_space(server_space) {
        Ok(_) => println!("{}", ADD_SUCCESS),
        Err(msg) => eprintln!("😔{}", msg)
    }
}

fn handle_command_list() {
    let server_space_list = Config::list_server_space();
    if server_space_list.is_empty() {
        println!("{}", SPACE_LIST_IS_EMPTY);
        return;
    }
    println!("{}", SPACE_LIST_TITLE);
    for name in server_space_list {
        println!("{}", Green.paint(name));
    }
}

fn handle_command_detail(arg_matches: &ArgMatches) {
    let server_space_name = arg_matches.get_one::<String>("space_name").unwrap();

    let server_space_option = Config::server_space_detail(server_space_name);
    match server_space_option {
        Some(server_space) => println!("{}", server_space),
        None => eprintln!("😔没有这个空间名称！")
    }
}

fn handle_command_remove(arg_matches: &ArgMatches) {
    let server_space_name = arg_matches.get_one::<String>("space_name").unwrap();
    match Config::remove_server_space(server_space_name) {
        Ok(_) => println!("{}", REMOVE_SUCCESS),
        Err(_) => eprintln!("{}", SPACE_NAME_IS_NOT_EXISTED)
    }
}

fn handle_command_push(arg_matches: &ArgMatches) {
    // 解析命令
    let pushed_dir = arg_matches.get_one::<String>("pushed_dir").unwrap();
    let server_space_name = arg_matches.get_one::<String>("space_name").unwrap();
    // 要推送的本地目录和要推送到的空间名称
    let pushed_dir = util::del_start_separator(pushed_dir).to_string();
    let server_space_name = server_space_name.to_string();

    // 要推送本地目录的绝对路径
    let pushed_dir_abs = PathBuf::from(env::current_dir().unwrap()).join(&pushed_dir);

    if !pushed_dir_abs.is_dir() {
        eprintln!("{}", IS_NOT_DIR);
        return;
    }

    // 要推送到的服务器空间
    let server_space = Config::server_space_detail(&server_space_name);
    if let Some(server_space) = server_space {
        // 进度条
        let pb = ProgressBar::new(100);
        pb.set_position(20);

        // 要推送的压缩文件名称和绝对路径
        let pushed_file_name = Arc::new(format!("{}.tar.gz", pushed_dir));
        let pushed_file_path = format!("{}.tar.gz", pushed_dir_abs.to_str().unwrap());

        // 打包压缩
        let pushed_file_name_copy = pushed_file_name.clone();
        let t = std::thread::spawn(move || {
            let tar_gz = File::create(pushed_file_name_copy.as_ref()).unwrap();
            let enc = GzEncoder::new(tar_gz, Compression::best());
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all("", pushed_dir).unwrap();
        });
        t.join().unwrap();

        pb.set_position(50);
        // 上传压缩文件到服务器
        if let Err(err) = push_file(&server_space, &pushed_file_name, &pushed_file_path) {
            eprintln!("{} {:?}", UPLOAD_ERR, err);
        } else {
            pb.finish();
            println!("{}", UPLOAD_SUCCESS);
        }


        // 删除本地压缩文件
        fs::remove_file(Path::new(&pushed_file_path)).unwrap();
    } else {
        eprintln!("{}", SPACE_NAME_IS_NOT_EXISTED);
    }
}

/// 建立服务器连接
fn get_ssh_session(server_space: &ServerSpace) -> Result<LocalSession<TcpStream>, SshError> {
    let pass = decrypt(&server_space.pass).unwrap();

    let session = ssh::create_session()
        .username(&server_space.user)
        .password(&pass)
        .connect(format!("{}:22", server_space.host))
        .unwrap()
        .run_local();

    Ok(session)
}

/// 上传文件到空间
fn push_file(server_space: &ServerSpace, pushed_file_name: &str, pushed_file_path: &str) -> Result<(), Box<dyn Error>> {
    // 获取ssh连接
    let mut session = get_ssh_session(server_space)?;
    // 上传压缩包
    let scp = session.open_scp()?;
    scp.upload(pushed_file_path, &server_space.path)?;

    // 目标服务器解压缩，解压缩后删除压缩文件
    session.open_exec()
        .unwrap()
        .send_command(&format!("cd {};tar zxf {};rm -rf {}", server_space.path, pushed_file_name, pushed_file_name))?;

    // 关闭连接
    session.close();
    Ok(())
}

/// 清空空间中的文件
fn handle_command_rmrf(arg_matches: &ArgMatches) {
    let server_space_name = arg_matches.get_one::<String>("space_name").unwrap();
    if let Some(server_space) = Config::server_space_detail(server_space_name) {
        println!("{}", Red.paint(RMRF_CONFIRM));
        let mut confirm = String::new();
        stdin().read_line(&mut confirm).unwrap();
        if let Ordering::Equal = confirm.to_lowercase().trim().cmp("yes") {
            let target_path = format!("{}/*", server_space.path);
            // 获取ssh连接
            let mut session = get_ssh_session(&server_space).unwrap();
            session.open_exec()
                .unwrap()
                .send_command(&format!("rm -rf {}", target_path))
                .unwrap();
            println!("{}", RMRF_SUCCESS);

            // 关闭连接
            session.close()
        }
    } else {
        eprintln!("{}", SPACE_NAME_IS_NOT_EXISTED);
    }
}
