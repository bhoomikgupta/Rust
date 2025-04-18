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
}