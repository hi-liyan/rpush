//! # command line arguments parse
//! 使用 clap crate 做命令解析处理

use clap::{Command, Arg, ArgMatches, ArgAction};

pub fn get_matches() -> ArgMatches {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        // 添加服务器空间配置
        .subcommand(Command::new("add")
            .about("Add server space config"))
        // 列出服务器空间配置
        .subcommand(Command::new("list").
            about("List server space config"))
        // 查看服务器空间详情
        .subcommand(Command::new("detail")
            .about("Print server space config detail")
            .arg(Arg::new("space_name")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .required(true)
            )
        )
        // 移除服务器空间配置
        .subcommand(Command::new("remove")
            .about("Remove server space config")
            .arg(Arg::new("space_name")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .required(true)
                .help("server space name")
            )
        )
        // 推送当前目录文件到服务器空间
        .subcommand(Command::new("push")
            .about("Push the specified directory under the current directory to the server")
            .arg(Arg::new("pushed_dir")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .required(true)
                .help("to be pushed dir"))
            .arg(Arg::new("space_name")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .required(true)))
        .subcommand(Command::new("rmrf")
            .about("Delete all dirs and files in the specified server space")
            .arg(Arg::new("space_name")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .required(true)
                .help("server space name")))
        .get_matches()
}