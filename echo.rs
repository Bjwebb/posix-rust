use std::env;

fn main() {
    for (i, argument) in env::args().skip(1).enumerate() {
        if i>0 { print!(" ") }
        print!("{}", argument.into_string().unwrap());
    }
    println!("");
}
