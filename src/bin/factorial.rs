// a program to write the factorial of a number
fn factorial(n: i32) -> i32 {
    if n == 0{
        1
    }
    else{
        n * factorial(n-1)
    }
}

fn main() {
    let num = 3;
    let result = factorial(num);
    println!("The factorial of {} is {}", num, result);
}