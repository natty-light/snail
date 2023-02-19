use std::io::{self, BufRead};

fn main() {
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
