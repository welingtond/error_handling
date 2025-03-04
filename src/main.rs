
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //let file = File::open("non_existent_file.txt");
    let path = std::env::args().skip(1).next().expect("No file path provided");
    println!("My path is: {}", &path);
    let file = File::open(path);

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}