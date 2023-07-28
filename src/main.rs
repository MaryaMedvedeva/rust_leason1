fn main() {
    // mathch



    let num = 87;

    match num {
        10 => println!("Num is 10"),
        23 => {
            println!("Num is 23");
            println!("Num is matched!")
        },
        0..=50 => {
            println!("Number betwen 0 and 50");
        },
        50..=100 => {
          println!("Number betven 50 and 100")
        },
        _ => {
            println!("No variants!");
            println!("Try again!");
        }
    }
}

