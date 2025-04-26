use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //panic!("This is a panic message");
    // let vec = vec![1, 2, 3];
    // vec[99]; // This will cause a panic
    // let file = File::open("non_existent_file.txt").unwrap();
    let file = File::open("non_existent_file.txt").expect("Failed to open the file");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("non_existent_file.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         }
    //         _=> panic!("Problem opening the file: {:?}", e),
    //     }
    // };
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
