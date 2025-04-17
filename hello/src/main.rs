fn main() {
    println!("Hello, world!");
    let mut x = 6;
    println!("the value of x is {}",x);
    x = 6;
    println!("the value of x is {}",x);
    const SECONDS:i64 = 60;
    println!("the value of SECONDS is {}",SECONDS);
    let y:u8 = 8;
    let decimal = 2.55;
    let hexa = 0x34;
    let binary = 0b1111;
    print!("{}\n",y);
    print!("{}\n",decimal);
    print!("{}\n", hexa);
    print!("{}\n", binary);
    let byte =b'A';
    println!("{}\n",byte);
    let _a = 5;
    let z = 2.0;//f64 default on modern CPU
    let t = true;
    let c = 'a';
    println!("{}\n",z);
    println!("{}\n",t);
    println!("{}\n",c);
    let s = x + _a;
    print!("{}",s);
}