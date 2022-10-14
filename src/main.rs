extern crate termion;

use termion::{color, style};

fn main() {
    println!("{}{}This Account can only connect using ftp!{}", style::Bold,color::Fg(color::Red),style::Reset);
    println!("{}{}Dieser Nutzer kann nur FTP verwenden!{}", style::Bold,color::Fg(color::Blue),style::Reset);
}


