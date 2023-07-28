fn main() {
    // match

    let numx =3;

    let num = match numx {
        2 => 1,
        3 => 10,
        3..=10 => 7,
        _ => 0
    };

    println!("{}", num);
}

