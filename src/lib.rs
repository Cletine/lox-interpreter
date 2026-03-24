pub mod lox_scanner;
use std::process;


pub fn error (line:usize, message: &str) -> () {
   report(line, "", message);
   process::exit(1);
}

fn report (line:usize, where_at:&str, message:&str) -> () {
    eprintln!("[Line {line} ] Error {where_at} : {message}");
}


