
use crate::models::command::Command;
use anyhow::{Result, anyhow};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use nix::fcntl::{flock, open, FlockArg, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::close;


// コマンドファイルパス取得
fn get_command_filepath(workspace_dir: std::path::PathBuf) -> std::path::PathBuf {
    workspace_dir.join(".command")
}

// コマンド書き込み
fn write_commands(workspace_dir: std::path::PathBuf, commands: Vec<Command>) -> Result<()> {
    // シリアライズ
    let serialized: String = serde_json::to_string(&commands).unwrap();

    // ファイルが無ければ作成
    let command_filepath = get_command_filepath(workspace_dir);
    OpenOptions::new().create(true).write(true).open(command_filepath.clone()).unwrap();

    // ロックして書き込み
    let fd = open(&command_filepath, OFlag::empty(), Mode::empty())?;
    match flock(fd, FlockArg::LockExclusive) {
        Ok(()) => {
            let mut file = File::create(command_filepath)?;
            file.write_all(serialized.as_bytes())?;
            close(fd)?;
        }
        Err(e) => {
            close(fd)?;
            return Err(e.into())
        }
    }

    Ok(())
}

// コマンド読み込み
pub fn load_commands(workspace_dir: std::path::PathBuf) -> Result<Vec<Command>> {
    // 存在しない場合はエラー
    let command_filepath = get_command_filepath(workspace_dir);
    if !command_filepath.clone().exists() {
        return Err(anyhow!("file not found"));
    }

    // ロックして読み込み
    let fd = open(&command_filepath, OFlag::empty(), Mode::empty())?;
    match flock(fd, FlockArg::LockExclusive) {
        Ok(()) => {
            let command_file = std::fs::OpenOptions::new()
                .read(true)
                .open(command_filepath)?;

            let reader = std::io::BufReader::new(command_file);
            let commands = serde_json::from_reader(reader)?;
            close(fd)?;
            Ok(commands)
        }
        Err(e) => {
            close(fd)?;
            return Err(e.into())
        }
    }
}

// コマンド確認
pub fn exists(workspace_dir: std::path::PathBuf, name: String) -> bool {
    // コマンド読み込み
    let commands = load_commands(workspace_dir.clone()).unwrap_or(Vec::new());

    // 存在確認
    commands.iter().any(|command| command.name==name)
}

// コマンド登録
pub fn register(workspace_dir: std::path::PathBuf, command: Command) -> Result<()> {
    // コマンド読み込み＆追加
    let mut commands = load_commands(workspace_dir.clone()).unwrap_or(Vec::new());
    if exists(workspace_dir.clone(), command.name.clone()) {
        let index = commands.iter().position(|registed_command| registed_command.name==command.name).unwrap_or(0);
        if commands.len() > index && index >= 0 {
            commands[index] = command;
        }
    }
    else {
        commands.push(command);
    }

    // コマンド書き込み
    write_commands(workspace_dir, commands)
}

// コマンド削除
pub fn delete(workspace_dir: std::path::PathBuf, name: String) -> Result<()> {
    // コマンド読み込み
    let mut commands = load_commands(workspace_dir.clone()).unwrap_or(Vec::new());

    // コマンド削除
    commands.retain(|command| *command.name != name);

    // コマンド書き込み
    write_commands(workspace_dir, commands)
}
