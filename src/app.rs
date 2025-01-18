use std::{
    char,
    io::{self, Read},
};

use crate::{signs, terminal_ops::*};
use termios::*;

pub fn run(choice: signs::SymblesChoice) {
    let stdin = 0;
    let original_termios = Termios::from_fd(stdin).unwrap();

    let mut new_termios = original_termios;
    new_termios.c_lflag &= !(ICANON | ECHO); //关闭回显和规范模式
    termios::tcsetattr(stdin, TCSANOW, &new_termios).unwrap();

    let mut reader = io::stdin();
    let mut buffer = [0; 1];

    loop {
        let symble: &'static char = signs::get_rand_symble(&choice);
        print_and_flush(&foreground_yellow(&symble.to_string()));

        //从标准输入读取一个字符
        reader.read_exact(&mut buffer).unwrap();
        let c = buffer[0] as char;

        clear_line();
        if c == *symble {
            print_and_flush(&foreground_green(&symble.to_string()));
        } else if c != 'q' {
            print_and_flush(&foreground_red(&symble.to_string()));
        } else {
            break;
        }
        println!();
    }
    termios::tcsetattr(stdin, TCSANOW, &original_termios).unwrap();
}
