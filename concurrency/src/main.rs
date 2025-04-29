use std:: thread;
use std:: time:: Duration;
fn main() {
    std::thread::spawn(|| {
        println!("Hello from a thread!");
    });
    //method 1
    // thread::sleep(Duration::from_secs(1));
    // println!("Hello from the main thread!");
    handle.join().unwrap();
}