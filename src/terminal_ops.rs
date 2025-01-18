//! 一些关于终端的操作
use std::io::{self, Write};

/// 清除终端这一行的内容,并把光标移动到最开始
pub fn clear_line() {
    let mut stdout = io::stdout();

    stdout.write_all(b"\x1b[2K\x1b[2D").unwrap();
    stdout.lock().flush().unwrap();
}

/// print并且刷新,立即显示
pub fn print_and_flush(content: &str) {
    let mut stdout = io::stdout();

    stdout.write_all(content.as_bytes()).unwrap();
    stdout.lock().flush().unwrap();
}

pub fn foreground_yellow(content: &str) -> String {
    format!("\x1b[33m{content}\x1b[0m")
}

pub fn foreground_green(content: &str) -> String {
    format!("\x1b[32m{content}\x1b[0m")
}

pub fn foreground_red(content: &str) -> String {
    format!("\x1b[31m{content}\x1b[0m")
}
