use std::io::stdin;

fn main() {
    let mut input = String::new();
    let ret = stdin().read_line(&mut input).unwrap();
    println!("{}", ret);
    if ret > 0 {
        input = input.trim().to_owned();
        println!("{}", input);
    }
    println!("Hello, world!");
}
