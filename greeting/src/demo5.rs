use std::io::stdin;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

fn test_read_line(){
    let mut str_buf = String::new();

    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}

fn test_file_read(){
    let text = fs::read_to_string("hello.txt").unwrap();
    println!("{}", text);
}

// fn test_file_write(){
//     fs::write("hello.txt", "FROM RUST PROGRAM")
//     .unwrap();
// }
// fn test_file_write(){
//     let mut file = File::create("hello.txt").unwrap();
//     file.write(b"FROM RUST PROGRAM AAA").unwrap();
// }
fn test_file_write() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
    .read(true).write(true).append(true).open("hello.txt")?;
    file.write(b" ssss APPEND WORD")?;
    Ok(())
}
fn main() {
    test_file_write();
}