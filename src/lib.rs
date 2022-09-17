//! # 主要处理流程
//! 从 `run()` 函数开始

#[macro_use]
extern crate clap;

use std::{
    cmp::Ordering,
    env,
    error::Error,
    fs::{self, File},
    io::stdin,
    path::{Path, PathBuf}
};

use clap::ArgMatches;
use flate2::{
    Compression,
    write::GzEncoder
};
use indicatif::ProgressBar;
use nu_ansi_term::Color::{Green, Red};
use ssh_rs::{
    Session,
    ssh,
    error::SshError
};

use crate::arg::get_matches;
use crate::config::{Config, ServerSpace};
use crate::utils as util;

mod config;
mod arg;
mod utils;

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
    let mut name = String::new();
    let mut host = String::new();
    let mut path = String::new();
    let mut user = String::new();
    let mut pass = String::new();

    println!("{}", Green.paint("输入空间名称"));
    stdin().read_line(&mut name).expect("read_line error!");
    if util::is_empty(&name) {
        eprintln!("😔空间名称不能为空！");
        return;
    }

    println!("{}", Green.paint("输入主机地址"));
    stdin().read_line(&mut host).expect("read_line error!");
    if util::is_empty(&host) {
        eprintln!("😔主机地址不能为空！");
        return;
    }

    println!("{}", Green.paint("输入目标路径"));
    stdin().read_line(&mut path).expect("read_line error!");
    if util::is_empty(&path) {
        eprintln!("😔目标路径不能为空！");
        return;
    }

    println!("{}", Green.paint("输入主机用户名"));
    stdin().read_line(&mut user).expect("read_line error!");
    if util::is_empty(&user) {
        eprintln!("😔主机用户名不能为空！");
        return;
    }

    println!("{}", Green.paint("输入主机密码"));
    stdin().read_line(&mut pass).expect("read_line error!");
    if util::is_empty(&pass) {
        eprintln!("😔主机密码不能为空！");
        return;
    }

    let server_space = ServerSpace::new(&name.trim(), &host.trim(),
                                        &path.trim(), &user.trim(), &pass.trim());
    match Config::add_server_space(server_space) {
        Ok(_) => println!("🎉添加成功️"),
        Err(msg) => eprintln!("😔{}", msg)
    }
}

fn handle_command_list() {
    let server_space_list = Config::list_server_space();
    if server_space_list.is_empty() {
        println!("😌还没有添加服务器空间");
        return;
    }
    println!("空间列表：");
    for name in server_space_list {
        println!("{}", Green.paint(name));
    }
}

fn handle_command_detail(arg_matches: &ArgMatches) {
    let server_space_name = arg_matches.value_of("space_name").unwrap();
    let server_space_option = Config::server_space_detail(server_space_name);
    match server_space_option {
        Some(server_space) => println!("{}", server_space),
        None => eprintln!("😔没有这个空间名称！")
    }
}

fn handle_command_remove(arg_matches: &ArgMatches) {
    let server_space_name = arg_matches.value_of("space_name").unwrap();
    match Config::remove_server_space(server_space_name) {
        Ok(_) => println!("🎉删除成功"),
        Err(_) => eprintln!("😔没有这个空间名称！")
    }
}

fn handle_command_push(arg_matches: &ArgMatches) {
    let pushed_dir = arg_matches.value_of("pushed_dir").unwrap();
    let server_space_name = arg_matches.value_of("space_name").unwrap();

    let pushed_dir = util::del_start_separator(pushed_dir).to_string();
    let server_space_name = server_space_name.to_string();

    let current_dir = PathBuf::from(env::current_dir().unwrap());
    let pushed_dir_abs = current_dir.join(&pushed_dir);

    if !pushed_dir_abs.is_dir() {
        eprintln!("😔无效的目录！");
        return;
    }

    let server_space_option = Config::server_space_detail(&server_space_name);
    if let Some(server_space) = server_space_option {
        // 进度条
        let pb = ProgressBar::new(100);
        pb.set_position(20);

        // 要推送的压缩文件名称和路径
        let pushed_file_name = format!("{}.tar.gz", pushed_dir);
        let pushed_file_path = format!("{}.tar.gz", pushed_dir_abs.to_str().unwrap());

        // 打包压缩
        let pushed_file_name_copy = pushed_file_name.clone();
        let pushed_dir_copy = pushed_dir.clone();
        let child = std::thread::spawn(move || {
            let tar_gz = File::create(pushed_file_name_copy).unwrap();
            let enc = GzEncoder::new(tar_gz, Compression::best());
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all("", pushed_dir_copy).unwrap();
        });
        child.join().unwrap();
        pb.set_position(50);

        // 上传压缩文件到服务器
        if let Err(_) = push_file(&server_space, &pushed_file_name, &pushed_file_path) {
            eprintln!("😔上传时发生错误，可能是空间配置信息不正确！");
        } else {
            pb.finish();
            println!("🎉上传成功");
        }


        // 删除本地压缩文件
        fs::remove_file(Path::new(&pushed_file_path)).unwrap();
    } else {
        eprintln!("😔没有这个空间名称！");
    }
}

/// 建立服务器连接
fn get_ssh_session(server_space: &ServerSpace) -> Result<Session, SshError> {
    let mut session: Session = ssh::create_session();
    session.set_timeout(15);
    session.set_user_and_password(&server_space.user, &server_space.pass);
    session.connect(format!("{}:22", server_space.host))?;
    Ok(session)
}

/// 上传文件到空间
fn push_file(server_space: &ServerSpace, pushed_file_name: &str, pushed_file_path: &str) -> Result<(), Box<dyn Error>>  {
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
    session.close()?;
    Ok(())
}

fn handle_command_rmrf(arg_matches: &ArgMatches) {
    let server_space_name = arg_matches.value_of("space_name").unwrap();
    if let Some(server_space) = Config::server_space_detail(server_space_name) {
        println!("{}", Red.paint("确认要删除空间中的所有文件？(yes继续，任意输入退出)"));
        let mut confirm = String::new();
        stdin().read_line(&mut confirm).unwrap();
        if let Ordering::Equal = confirm.to_lowercase().trim().cmp("yes") {
            let target_path = format!("{}/*", server_space.path);
            // 获取ssh连接
            let mut session: Session = get_ssh_session(&server_space).unwrap();
            session.open_exec()
                .unwrap()
                .send_command(&format!("rm -rf {}", target_path))
                .unwrap();
            println!("🎉空间文件已全部清除");
        }
    } else {
        eprintln!("😔没有这个空间名称！");
    }
}
