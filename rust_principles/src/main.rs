fn main() {
    // println!("Hello, world!");
    //let var = 1;
    let mut s = "hello".to_string(); //created on the heap
    s.push_str(" world");
    println!("{}", s);
    //move
    // let x = vec![1, 2, 3];
    // let y = x; // x is moved to y
    // // println!("{:?}", x); // This will cause a compile-time error
    // println!("{:?}", y); // This will work

    //clone
    let x = vec![1, 2, 3]; //this is created on the heap
    let y = x.clone(); // x is cloned to y
    println!("{:?}", x); // This will work
    println!("{:?}", y); // This will work

    //copy
    let x = 1; // this is created on the stack
    let y = x; // x is copied to y
    println!("{:?}", x); // This will work
    println!("{:?}", y); // This will work
    let s = String::from("hello");
    takes_ownership(s); // give ownership of s to the function
    // println!("{}", s); // This will cause a compile-time error
}
// s is dropped here
//var is dropped after main
fn takes_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
    // s is dropped here
}

fn make_copy(x: i32) {
    let val1 = x;
    println!("{}", val1);
}
