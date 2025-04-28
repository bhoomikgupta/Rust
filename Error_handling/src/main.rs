use std::fs::File;
use std::io::ErrorKind;
use std::fs::rename;
use std::io::Error;
fn main() {
    //panic!("This is a panic message");
    // let vec = vec![1, 2, 3];
    // vec[99]; // This will cause a panic
    // let file = File::open("non_existent_file.txt").unwrap();
    // let file = File::open("non_existent_file.txt").expect("Failed to open the file");
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
    let test = open_file();
    test.unwrap();
    rename_file().unwrap();
}
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}
fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    Ok(file)
}
fn rename_file() -> Result<(), Error> {
    rename("error.txt", "rename.txt")?;
    Ok(())
}