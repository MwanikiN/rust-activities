fn main() {
    for i in 1..=100 {
        match i {
            i if i % 15 == 0 => println!("FizzBuzz"),
            i if i % 3 == 0 => println!("Fizz"),
            i if i % 5 == 0 => println!("Buzz"),
            _ => println!("{}", i),
        }
        }
    }
