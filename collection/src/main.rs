use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BinaryHeap;
fn main() {
    let mut num: Vec<i32> = vec![1, 2, 3, 4, 5];
    num.push(6);
    num.push(7);
    num.push(8);
    let pop = num.pop();
    println!("Popped value: {:?}", pop);
    let two = num[1]; //copy
    println!("Second value: {:?}", two);
    let one = num.first(); //reference
    println!("First value: {:?}", one);
    //.last
    let last = num.last();
    println!("Last value: {:?}", last);
    //.get
    let get = num.get(2);
    println!("Get value: {:?}", get);
    //.remove
    let remove = num.remove(2);
    println!("Removed value: {:?}", remove);
    //.insert
    num.insert(2, 10);
    println!("Inserted value: {:?}", num);
    //.extend
    let mut num2: Vec<i32> = vec![11, 12, 13];
    num.extend(num2);
    println!("Extended value: {:?}", num);
    //.shuffle
    let mut rng = thread_rng();
    num.shuffle(&mut rng);
    println!("Shuffled value: {:?}", num);
    //.copy_within
    num.copy_within(0..3, 3);
    println!("Copied within value: {:?}", num);
    //.swap
    num.swap(0, 1);
    println!("Swapped value: {:?}", num);
    //.sort
    num.sort();
    println!("Sorted value: {:?}", num);
    //.reverse
    num.reverse();
    println!("Reversed value: {:?}", num);
    //.clear
    num.clear();
    println!("Cleared value: {:?}", num);
    //.is_empty
    let is_empty = num.is_empty();
    println!("Is empty: {:?}", is_empty);
    //.len
    let len = num.len();
    println!("Length: {:?}", len);
    //.capacity
    let capacity = num.capacity();
    println!("Capacity: {:?}", capacity);
    //.resize
    num.resize(10, 0);
    println!("Resized value: {:?}", num);
    //.retain
    num.retain(|&x| x > 5);
    println!("Retained value: {:?}", num);
    let mut bheap = BinaryHeap::new();
    bheap.push(1);
    bheap.push(2);
    bheap.push(3);
    bheap.push(4);
    println!("Binary heap: {:?}", bheap);
    bheap.pop();
    println!("Binary heap after pop: {:?}", bheap);
    println!("Binary heap after pop: {:?}", bheap.peek());//peak is going to return option<T> , return none if empty
}
