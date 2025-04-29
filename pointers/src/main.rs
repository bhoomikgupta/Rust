use std::rc::Rc;
use std::cell::RefCell;
struct Flagger {
    is_true: Rc<RefCell<bool>>,
}
fn main() {
    let t = (12,"eggs");//created on the stack
    let b = Box::new(t);// created on the heap, but b was created on the stack
    println!("Tuple: {:?}", b);
    let x = 5;
    let y = &x; // y is a reference to x
    assert_eq!(x, *y); // dereference y to get the value of x
    println!("x: {}, y: {}", x, y);
    let x = 5;
    let y = Box::new(x); // y is a Box pointing to x
    assert_eq!(x, *y); // dereference y to get the value of x
    println!("x: {}, y: {}", x, *y);
    let s1 = Rc::new(String::from("Hello"));
    let s2 = s1.clone(); // s2 is a clone of s1, but they point to the same memory location
    let s3 = s2.clone(); // s3 is a clone of s2, but they point to the same memory location
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
    let flag = Flagger {
        is_true: Rc::new(RefCell::new(true)),
    };
    //borrow returns Ref<T>
    //borrow_mut returns RefMut<T>
    // let reference = flag.is_true.borrow();
    // println!("Flag is: {}", *reference);
    let flag =  Flagger {
        is_true: Rc::new(RefCell::new(true)),
    };
    let reference = Rc::new(flag.is_true.clone());
    println!("Flag is: {:?}", reference);

    //let mut mut_ref = flag.is_true.borrow_mut();
    let mut mut_ref = reference.borrow_mut();
    *mut_ref = false;
    println!("Flag is: {}", *mut_ref);
}
