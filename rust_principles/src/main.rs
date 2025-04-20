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
    let val1 =1;
    make_copy(val1); // copy val1 to the function
    println!("{}", val1); // This will work
    let str1:String = give_ownership();
    println!("{}", str1); // This will work
    let str3:String = take_and_give_ownership(str1);
    println!("{}", str3); // This will work
    let str4 = String::from("Bhoomik");
    let mut str5:String;
    // loop{
    //     //str5=str4; //value moved here, in previous iteration of loop
    //     str5 = str4.clone(); //value copied here, in previous iteration of loop
    // }
    let mut str6 = String::from("hello");
    change_string(&mut str6); // pass a reference to s
    println!("{}",str6);
}
// s is dropped here
//var is dropped after main
fn change_string(some_string: &mut String) {
    some_string.push_str(" world"); // This will cause a compile-time error
}

fn takes_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
    // s is dropped here
}

fn make_copy(x: i32) {
    let val1 = x;
    println!("{}", val1);
}

fn give_ownership() -> String {
    "given".to_string()
}
fn take_and_give_ownership(s: String) -> String {
    s
 }