fn main() {
    // match

    let is_old = false;

    let mut redy_num: String = String::new();

    match is_old {
       true => {
            redy_num = String::from("Come in!");
        },
        false => {
            redy_num = String::from("Go out!")
        },
    }

    println!("{}", redy_num);
}

