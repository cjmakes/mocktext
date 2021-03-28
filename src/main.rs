use std::io;

use mocktext::mock;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    println!("{}", &mock(&input.trim_matches('\u{000a}').to_string()));
}
