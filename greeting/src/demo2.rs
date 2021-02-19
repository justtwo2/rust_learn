use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }
}
// fn main() {
//     let f1 = File::open("hello.txt").unwrap();
//     let f2 = File::open("hello.txt").expect("Failed to open.");
    
// }