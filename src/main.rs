#![allow(warnings)]
use nether::rustpp::*;

fn main() {
    let path = "test.n".to_string();
    let f = read_file(&path);
    let mut _start_list = Vec::new();
    // println!("{}",f);
    let tokens:Vec<String> = f.split(';').map(|s|s.to_string()).collect();
    println!("token: {:?}",tokens);
    for token in tokens{
        let (cmd, val) = token.split_once(" ").unwrap();
        if cmd == "return"{
            _start_list.push("mov  rax, 60");
            _start_list.push("mov rdi, 0");
            _start_list.push("syscall");
        }
    }

}
