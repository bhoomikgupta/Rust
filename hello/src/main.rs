fn bhoomik() {
    println!("Hello Bhoomik!");
}
fn print_bhoomik(phrase: &str) {
    println!("{}", phrase);
}
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)//leaving semicolon meaning return value
    }
}

fn multiple_return_values(flag:bool) -> bool {
    if flag {
        return true;
    } else {
        return false;
    }
}
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
    println!("\n");
    //tuples
    let tup = (1,"Bhoomik");
    println!("{}\n",tup.0);
    println!("{}\n",tup.1);
    let (x,y) = tup;
    println!("{}\n",x);
    println!("{}\n",y);
    //arrays
    let mut array = [1,2,3];
    println!("{}\n",array[0]);
    println!("{}\n",array[1]);
    array[2] = 4;
    println!("{}\n",array[2]);
    println!("{}\n",array.len());
    //println!("{}\n",array[3]);
    //|^^^^^^^^ index out of bounds: the length is 3 but the index is 3
    
    //vectors
    let mut nums = vec![2,3];
    nums.push(4);
    println!("{:?}\n",nums);
    nums.pop();
    println!("{:?}\n",nums);
    let mut vec = Vec::new();
    vec.push("ashish");
    vec.push("ashish1");
    vec.push("Bhoomik");
    println!("{:?}\n",vec);
    vec.reverse();
    println!("{:?}\n",vec);
    let mut vect = Vec::<i32>::with_capacity(2);
    vect.push(1);
    vect.push(2);
    vect.push(3);
    println!("{:?}\n",vect);
    println!("{:?}\n",vect.capacity());
    let v: Vec<i32> = (0..10).collect();
    println!("{:?}\n",v);
    //slicing
    let slice = &v[0..5];
    println!("{:?}\n",slice);
    let slice1 =&v[2..4];
    println!("{:?}\n",slice1);
    //string
    let name = String::from("Bhoomik");
    println!("{:?}\n",name);
    let course = "CSE".to_string();
    println!("{:?}\n",course);
    let new_name =name.replace("Bhoomik","Ashish");
    println!("{:?}\n",new_name);
    let str1 = "Hello";
    let str2 = str1.to_string();
    let str3 = &str2;
    println!("{:?}\n",str3);
    println!("{:?}\n",str2);
    print!("{}\n",str1);
    //compare strings
    println!("{:?}\n",str1 == str2);
    
    //functions
    bhoomik();
    print_bhoomik("Hello Bhoomik!");
    println!("{}", gcd(20, 5));  
    println!("{}", multiple_return_values(true));

    //control flow
    // loop{
    //     println!("Hello Bhoomik!");
    // } infinite loop
    let mut num = 0;
    'counter:loop{
        println!("Counter: {}",num);
        let mut decrease = 5;
        loop{
            println!("Decreasing: {}",decrease);
            decrease -= 1;
            if decrease == 0{
                break;
            }
            if num == 2{
                break 'counter;
            }
        }
        num += 1;
    }
    let mut num =0;
    while num < 5{
        println!("Counter: {}",num);
        num += 1;
    }
    let vec: Vec<i32> = (0..10).collect();
    for i in &vec{
        println!("Counter: {}",i);
    }
    for number in (1..4).rev(){
        println!("Counter: {}",number);
    }
}