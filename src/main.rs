use std::env;
use std::path::{Path, PathBuf};
use colored::Colorize;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let args: Vec<String> = env::args().collect();
    let working_dir = env::current_dir().unwrap();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    
    if args.len() <= 1 {
        // 无参数情况，仅输出目录
        let msg = working_dir.to_str().unwrap();
        println!("{}", msg.cyan());
        // 复制到剪切板
        ctx.set_contents(msg.to_string()).unwrap();
    } else {
        // 有参数情况
        let raw_arg_str = args[1].clone();
        let full_path = Path::join(&*working_dir.clone(), PathBuf::from(raw_arg_str.clone()));
        let new_path = PathBuf::from(full_path.clone());
        if new_path.exists() {
            // 真实存在的文件
            let _path = new_path.canonicalize().unwrap();
            let _path_str = _path.to_str().unwrap();
            let mut _striped_path = _path_str.clone();
            // 去除 Windows 长目录标识符
            if _path_str.starts_with(r"\\?\") {
                _striped_path = _path_str.strip_prefix(r"\\?\").unwrap();
            }
            if _path.is_dir() {
                // 目录为青色
                println!("{}", _striped_path.cyan());
            } else {
                // 文件为绿色
                println!("{}", _striped_path.green());
            }
            // 复制到剪切板
            ctx.set_contents(_striped_path.to_string()).unwrap();
            // } else {
            // // 不存在的文件
            // let mut fake_file_path = raw_arg_str.clone();
            // if raw_arg_str.starts_with(r".\") {
            //     fake_file_path = raw_arg_str.strip_prefix(r".\").unwrap().to_string();
            // }
            // let full_path = Path::join(&*working_dir.clone(), PathBuf::from(fake_file_path));
            // println!("{}", full_path.display());
        }
    }
}
