use async_std::task::Poll;
use async_std::fs::File;
use async_std::io;
use async_std::prelude::*;
use async_std::task;
async fn read_file(path: & str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
    //the async keyword is used to define an asynchronous function
    //the await keyword is used to wait for the result of an asynchronous operation
    //the ? operator is used to propagate errors
    //the async_std crate provides the async_std::fs module for file operations
    //the async_std::io module provides the async_std::io::prelude module for I/O operations

fn main() {
    println!("Hello, world!");
    let task = task::spawn(async{
        let result = read_file("read.txt").await;
        match result {
            Ok(k) => println!("{}", k),
            Err(e) => println!("Error: {}", e),
        }
    });
    println!("Task has started");
    task::block_on(task);
    println!("Task has finished");
}
// trait Future {
//     type Output;
//     fn poll(self::Pin(&mut self), cx: &mut Context) -> Poll<Self::Output>;
//     //poll is a method that is called to check if the future is ready to produce a value
//     //it takes a mutable reference to self and a context
//     //it returns a Poll enum which can be either Ready or Pending
//     //Ready means the future is ready to produce a value
//     //Pending means the future is not ready yet
//     //the context is used to wake up the future when it is ready
//     //poll::ready poll::pending
// }