use std::io;

fn main() {
    // io
    // i - input
    // o - output

    let mut name = String::new();

    println!("PLese, write your name: ");

    match  io::stdin().read_line(&mut name){
         Ok(_) => {
            println!("Hello, {}", name);
         },
        Err(e) => {
            println!("PROGRAM ERROR - {}", e);
        }
    }
}

