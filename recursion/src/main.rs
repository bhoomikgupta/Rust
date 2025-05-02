fn fact(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }
    num * fact(num - 1)
}

fn fib(num: u32) -> u32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }
    fib(num - 1) + fib(num - 2)
}

fn palindrome(array: &Vec<i32>, start: usize, end: usize) -> bool {
    if start >= end {
        return true;
    }
    if array[start] != array[end] {
        return false;
    }
    palindrome(array, start + 1, end - 1)
}
fn main() {
    let num = 5;
    let result = fact(num);
    println!("The factorial of {} is {}", num, result);
    let num = 5;
    let result = fib(num);
    println!("The fibonacci of {} is {}", num, result);
    let array = vec![1, 2, 3, 2, 1];
    let start = 0;
    let end = array.len() - 1;
    let result = palindrome(&array, start, end);
    if result {
        println!("The array is a palindrome");
    } else {
        println!("The array is not a palindrome");
    }
}
