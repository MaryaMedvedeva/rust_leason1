use std::intrinsics::transmute;
use std::io;

fn main() {
    // ax^2 + bx + c = 0
    // D = b^2 - 4(a*c)

    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    loop {
        println!("Solve quadratic equation");

        println!("Write a:");
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {},
            Err(e) => println!("Input error - {}", e)
        }

        println!("Write b:");
        match io::stdin().read_line(&mut b_str) {
            Ok(_) => {},
            Err(e) => println!("Input error - {}", e)
        }

        println!("Write c:");
        match io::stdin().read_line(&mut c_str) {
            Ok(_) => {},
            Err(e) => println!("Input error - {}", e)
        }

        let a: f64 = a_str.trim().parse().unwrap();
        let b: f64 = b_str.trim().parse().unwrap();
        let c: f64 = c_str.trim().parse().unwrap();

        let d: f64 = (b * b) - 4.0 * (a * c);

        if d > 0.0 {
            let x1 = ((-b) + d.sqrt()) / (2.0 * a);
            let x2 = ((-b) - d.sqrt()) / (2.0 * a);

            println!("Answer\nThere are two roots\nD = {}\nRoot 1 = {}\nRoot 2 = {}", d, x1, x2);
        }
        if d == 0.0 {
            let x = (-b) / (2.0 * a);

            println!("Answer\nThere are one roots\nD = 0\nRoot = {}", x);
        }

        if d < 0.0 {
            println!("Roots don't exist\nD < 0\n = {}", d);
        }
    }

}

