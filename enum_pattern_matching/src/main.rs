enum Pet {
    Dog,
    Cat,
    Fish,
}
impl Pet {
    fn sound(&self) -> &str {
        match self {
            Pet::Dog => "Woof",
            Pet::Cat => "Meow",
            Pet::Fish => "Blub",
        }
    }
}

enum IpAddrKind{
    V4,
    V6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

// enum Option<T> {
//     Some(T),
//     None,
// }
fn main() {
    let dog = Pet::Dog;
    println!("Dog sound: {}", dog.sound());
    let cat = Pet::Cat;
    println!("Cat sound: {}", cat.sound());
    let _home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    let _x:i32 = 5;
    let _y: Option<i32> = Some(10);
    // let sum = x + y; // This will cause a compile-time error
    //no implementation for `i32 + Option<i32>
    let five = Some(5);
    let six = plus_one(five);
    println!("Five plus one: {:?}", six);
    let none = plus_one(None);
    println!("None plus one: {:?}", none);
    what_pet("dog");
    what_pet("cat");
    what_pet("fish");
    what_pet("bird"); // This will panic at runtime

}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn what_pet(input: &str) {
    match input {
        "dog" => println!("Dog"),
        "cat" => println!("Cat"),
        "fish" => println!("Fish"),
        _ => panic!("Unknown pet!"),
    }
}
