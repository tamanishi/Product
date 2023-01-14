use std::io;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let values = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    match (values[0] * values[1]) % 2 {
        0 => println!("Even"),
        1 => println!("Odd"),
        _ => (),
    }
}
