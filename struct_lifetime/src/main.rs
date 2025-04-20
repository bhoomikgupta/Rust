struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32); // Tuple struct
struct UnitStruct; // Unit struct
struct Square {
    width: u32,
    height: u32,
}
impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn whats_my_width(&self) -> u32 {
        self.width
    }
    fn whats_my_height(&self) -> u32 {
        self.height
    }

    fn change_width(&mut self, width: u32) {
        self.width = width;
    }
}


fn main() {
    let user1 = User {
        active: true,
        username: String::from("bhoomik"),
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        sign_in_count: user1.sign_in_count,
    };
    // println!("User 1: {} {} {}", user1.active, user1.username, user1.sign_in_count); value borrowed here after move
    println!("User 2: {} {} {}", user2.active, user2.username, user2.sign_in_count);
    println!("Hello, world!");
    let _user3 = build_user(String::from("bhoomik"));
    let cords = Coordinates(1, 2, 3);
    println!("Coordinates: ({}, {}, {})", cords.0, cords.1, cords.2);
    let mut sq = Square {
        width: 10,
        height: 20,
    };
    println!("Area of square: {}", sq.area());
    println!("Width of square: {}", sq.whats_my_width());
    println!("Height of square: {}", sq.whats_my_height());
    sq.change_width(30);
    println!("New Width of square: {}", sq.whats_my_width());
}

fn build_user(username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 1,
    }
}