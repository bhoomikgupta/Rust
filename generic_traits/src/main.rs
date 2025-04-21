struct Point<T,U> {
    x: T,
    y: U,
}
trait Overview {
    fn overview(&self) -> String {
        String::from("This is a rust course")
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
}
