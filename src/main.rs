mod models;
mod command_manager;

use crate::models::cli::Cli;
use crate::models::cli::SubCommand;
use crate::models::command::Command;
use std::io::{stdin, stdout, Write};
use anyhow::{Result, anyhow};
use clap::Parser;

fn main() -> Result<()> {
    // コマンドマネージャー作成
    let workspace_dir = get_workspace();

    // コマンド解析
    let cli = Cli::parse();
    match cli.subcommand {
        SubCommand::List => show(workspace_dir),
        SubCommand::Register {
            name,
            command,
            description,
        } => register(workspace_dir, name, command, description),
        SubCommand::Delete { name } => delete(workspace_dir, name)
    }
}

// ワークスペース取得
fn get_workspace() -> std::path::PathBuf {
    dirs::home_dir().unwrap_or(std::path::PathBuf::new())
}

// 一覧表示
fn show(workspace_dir: std::path::PathBuf) -> Result<()> {
    let commands = command_manager::load_commands(workspace_dir.clone()).unwrap_or(Vec::new());
    println!(
        "{0: <5} | {1: <10} | {2: <20} | {3: <30}",
        "No", "name", "command", "description"
    );
    for (i, command) in commands.into_iter().enumerate() {
        let name = command.name;
        let command_ = command.command;
        let description = command.description;
        println!("{i: <5} | {name: <10} | {command_: <20} | {description: <30}");
    };
    Ok(())
}

// コマンド登録
fn register(
    workspace_dir: std::path::PathBuf,
    name: Option<String>,
    command: Option<String>,
    description: Option<String>,
) -> Result<()> {
    // 入力されていない場合は入力を求める
    let mut regist_name: String = name.unwrap_or(String::from(""));
    {
        if regist_name == "" {
            print!("name: ");
            stdout().flush().unwrap();
            let mut result = String::new();
            stdin().read_line(&mut result).expect("input failed");
            regist_name = String::from(result.trim());
        }
        if regist_name == "" {
            return Err(anyhow!("name cannot be empty"));
        }
    }
    let mut regist_command: String = command.unwrap_or(String::from(""));
    {
        if regist_command == "" {
            print!("command: ");
            stdout().flush().unwrap();
            let mut result = String::new();
            stdin().read_line(&mut result).expect("input failed");
            regist_command = String::from(result.trim());
        }
        if regist_command == "" {
            return Err(anyhow!("command cannot be empty"));
        }
    }
    let mut regist_description: String = String::from("");
    {
        if description.is_none() {
            print!("description(option): ");
            stdout().flush().unwrap();
            let mut result = String::new();
            stdin().read_line(&mut result).expect("input failed");
            regist_description = String::from(result.trim());
        }
        else {
            regist_description = description.unwrap_or(String::from(""));
        }
    }

    // コマンド作成
    let command = Command {
        name: regist_name.clone(),
        command: regist_command,
        description: regist_description,
    };

    // 存在確認、上書きチェック
    let mut is_overwrite: bool = true;
    if command_manager::exists(workspace_dir.clone(), regist_name.clone()) {
        print!("Already exists. Overwrite it?(y/n): ");
        stdout().flush().unwrap();
        let mut result = String::new();
        stdin().read_line(&mut result).expect("input failed");
        is_overwrite = result.trim() == String::from("y");
    }

    // 登録
    if is_overwrite {
        command_manager::register(workspace_dir, command)?;
        println!("registed {regist_name}");
    }
    Ok(())
}

// コマンド削除
fn delete(workspace_dir: std::path::PathBuf, name: String) -> Result<()> {
    command_manager::delete(workspace_dir, name.clone())?;
    println!("delete {name}");
    Ok(())
}
