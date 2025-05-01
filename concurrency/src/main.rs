use std:: thread;
use std:: time:: Duration;
use std:: sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
// use num::BigUint;
use num::One;
use rayon::prelude::*;
use num::{BigUint, one};
use std::time::Instant;
fn factorial(num:u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    } else {
        (1..num).map(BigUint::from).reduce(|acc, x| acc * x).unwrap()
    }
}

fn multi_fact(num:u32) -> BigUint {
    if num ==0 || num  == 1 {
        return BigUint::one()
    } else {
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc * x)
    }
}
fn main() {
    let now = Instant::now();
    factorial(100000);
    println!("{:.2?}", now.elapsed());
    let now = Instant::now();
    multi_fact(100000);
    println!("{:.2?}", now.elapsed());
    let v = vec![1,2,3];
    // let handle = std::thread::spawn(move|| {
    //     println!("{:?}",v);
    // });
    let mut thread_handles = Vec::new();
    for i in v {
        thread_handles.push(thread::spawn(move || println!("Thread {}", i)));
    }
    //method 1
    thread::sleep(Duration::from_secs(1));
    println!("Hello from the main thread!");
    println!("Hello from the main thread!");
    for handle in thread_handles {
        handle.join().unwrap();
    }
    // let (transmitter, receiver) = mpsc::channel();
    let (transmitter, receiver) = mpsc::sync_channel(1000);
    let tx = transmitter.clone();
    // let val = String::from("Hello");
    // std::thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });
    // let msg = receiver.recv().unwrap();
    // println!("Received: {}", msg);
    std::thread::spawn(move || {
        let vec = vec![String::from("Transmitting"),
                        String::from("from"),
                        String::from("Original")];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });
    std::thread::spawn(move || {
        let vec = vec![String::from("Clone"),
                        String::from("is"),
                        String::from("transmitting")];
        for val in vec {
            tx.send(val).unwrap();
        }
    });
    for rec in receiver {
        println!("Received: {}", rec);
    }
    let rc1 = Arc::new(String::from("Hello"));
    let rc2 = rc1.clone();
    std::thread::spawn(move || {
        println!("Thread 1: {}", rc1);
    });
    std::thread::spawn(move || {
        println!("Thread 2: {}", rc2);
    });
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            //let mut num2 = counter.lock().unwrap(); deadlock
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    //mutex_poisoning
    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);
    let _ = std::thread::spawn(move || -> () {
        let _guard = lock2.lock().unwrap();
        panic!("Mutex poisoned");
    }).join();
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            println!("Mutex poisoned");
            poisoned.into_inner()
        }
    };
    *guard += 1;
    println!("Result: {}", *guard);
}