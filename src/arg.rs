//! command line arguments parse

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_matches() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        // 添加服务器空间配置
        .subcommand(SubCommand::with_name("add")
            .about("add server space config"))
        // 列出服务器空间配置
        .subcommand(SubCommand::with_name("list").
            about("list server space config"))
        // 查看服务器空间详情
        .subcommand(SubCommand::with_name("detail")
            .about("server space config detail")
            .arg(Arg::with_name("space_name")
                .required(true)))
        // 移除服务器空间配置
        .subcommand(SubCommand::with_name("remove")
            .about("remove server space config")
            .arg(Arg::with_name("space_name")
                .required(true)
                .help("server space name")))
        // 推送当前目录文件到服务器空间
        .subcommand(SubCommand::with_name("push")
            .about("push current directory files to server space")
            .arg(Arg::with_name("space_name")
                .required(true)))
        .get_matches()
}