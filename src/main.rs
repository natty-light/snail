use std::{io::{self, BufRead}, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => snail(),
        2 => execute_action(&args[1]),
        _ => help(),
    }
}

fn help() {
    let help_string = "Insert Useful information here";
    println!("{}", help_string)
}

fn snail() {
    let stdin = io::stdin();
    let snail = r#"
             \          @             _________
                         \____       /         \
                         /    \     /    ____   \
                         \_    \   /    /    \   \
                           \    \ (    \__/  )   )
                            \    \_\ \______/   /
                             \      \          /___
                              \______\________/____-_"#;

    for line in stdin.lock().lines() {
        println!("{}", line.unwrap())
    }

    println!("{}", snail);
}

fn trail() {
    println!("{}", "Implement trail feature")
}

fn execute_action(arg: &str) {
    match arg {
        "help" => help(),
        "trail" => trail(),
        &_ => println!("Unknown argument {}", arg)
    }
}
