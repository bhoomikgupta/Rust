struct Point<T,U> {
    x: T,
    y: U,
}
trait Overview {
    fn overview(&self) -> String {
        String::from("This is a rust course")
    }
}

impl Drop for Course{
    fn drop(&mut self) {
        println!("Dropping Course: {}", self.headline);
    }
}

trait Clone: Sized{
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}
struct Course {
    headline: String,
    description: String,
}

struct AnotherCourse {
    headline: String,
    description: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{}: {}", self.headline, self.description)
    }
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}: {}", self.headline, self.description)
    }
}
use std::Ops::Add;
struct Point2D<T> {
    x: T,
    y: T,
}
fn main() {
    let coordinates = Point { x: 5, y: 10 };
    let coordinates_float = Point { x: 5.0, y: 10.0 };
    let coordinates_string = Point {
        x: String::from("Hello"),
        y: String::from("World"),
    };
    let coordinates_mixed = Point {
        x: 5,
        y: String::from("Hello"),
    };
    let course1 = Course {
        headline: String::from("Rust Programming"),
        description: String::from("Learn Rust from scratch"),
    };
    let course2 = AnotherCourse {
        headline: String::from("Advanced Rust"),
        description: String::from("Deep dive into Rust"),
    };
    println!("Course 1: {}", course1.overview());
    println!("Course 2: {}", course2.overview());
    println!("Coordinates: x = {}, y = {}", coordinates.x, coordinates.y);
    println!("Coordinates Float: x = {}, y = {}", coordinates_float.x, coordinates_float.y);
    println!("Coordinates String: x = {}, y = {}", coordinates_string.x, coordinates_string.y);
    println!("Coordinates Mixed: x = {}, y = {}", coordinates_mixed.x, coordinates_mixed.y);
    call_overview(&course1);
}
// This function takes a reference to an item that implements the Overview trait
fn call_overview<T:Overview>(item: &T) {
    println!("{}", item.overview());
}
// fn call_overview(item:&impl Overview) {
//     println!("{}", item.overview());
// }

// fn call_overview<T:Overview>(item1: &T, item2: &T)
// fn overview(item1: &impl Overview, item2: &impl Overview) {
//     println!("{} {}", item1.overview(), item2.overview());
//}
//fn overview(item1: &impl Overview + AnotherTraits)
//fn overview<T: Overview + AnotherTraits>(item1: &T, item2: &T) {
