mod list {
    pub struct Tasks {
        pub item: String,
    }
    // mod things_todo{
    //     fn add_activity(){}
    //     fn update_activity(){}
    //     fn marked_complete(){}
    // }
    // mod items_done{
    //     fn remove_activity(){}
    //     fn update_activity(){}
    // }
}
mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;
use things_todo::items_completed::test::test;
fn lets_add_task(){
    let task = list::Tasks{
        item: String::from("Learn Rust"),
    };
    println!("Task: {}", task.item);
    // list ::things_todo::add_activity(); //relative path
    // crate::list::things_todo::add_activity(); //absolute path
    add_activity(); //imported function
    items_completed::remove_activity(); //imported function
    test(); //imported function
}