#[derive(Debug)]
struct City {
    city: String,
    population:u64,
}

fn sort_pop(city:& mut Vec<City>){
    city.sort_by_key(pop_helper);
}

fn pop_helper(pop: &City) -> u64 {
    pop.population
}
fn sort_pop_closure(pop: &mut Vec<City>) {
    pop.sort_by_key(|p| p.population);
}
fn main() {
   let a = City{city: String:: from("A"), population: 300};
   let b = City{city: String:: from("B"), population: 200};
   let c = City{city: String:: from("C"), population: 600};
   let mut vec: Vec<City> = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    //sort_pop(&mut vec);
    sort_pop_closure(&mut vec);
    println!("Sorted cities by population:");
    println!("{:?}", vec);
    let add = |x: i32| -> i32 { x + 1 };
    println!("add(1): {}", add(1)); // Output: add(1): 2
    
    let add2 = |x| x + 1;
    println!("add2(1): {}", add2(1)); // Output: add2(1): 2
    
    let example = |x| x;
    let num = example(2);
    println!("example(2): {}", num); // Output: example(2): 2
    // let string = example(String::from("Hello"));
    // println!("example(String): {}", string); // Output: example(String): Hello
    
    //fn, FnOnce, FnMut
    // || drop(v) FnOnce
    // |args| v.contains(args) Fn
    // |args| v.push(args) FnMut

    let y = 5;
    let add_y = |x| x + y;
    let copy = add_y;
    print!("{}",add_y(copy(1))); // Output: 6

    let mut y = 5;
    let mut add_y = |x| {
        y += x;
        y
    };
    let copy = add_y;
    // print!("{}",add_y(copy(1))); // Output: 7 error
}
