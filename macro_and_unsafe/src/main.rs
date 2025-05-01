macro_rules! gcd {
    ($a:expr, $b:expr) => {{
        let mut m = $b;
        let mut n = $a;
        while m!=0 {
            if m<n {
                let t = m;
                m = n;
                n = t ;
            }
            m = m % n;
        }
        n
    }};
}
fn main() {
    println!("{}", gcd!(14,28));
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
    println!("r1 is {:?}",*r1);
    println!("r1 is {:?}",*r2);
    }
}
