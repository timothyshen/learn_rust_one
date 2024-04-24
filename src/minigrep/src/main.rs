use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    print!("Searching for {}", query);
    print!("In file {}", file_path);

    let contents  = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}
