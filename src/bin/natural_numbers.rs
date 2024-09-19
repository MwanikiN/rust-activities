// sum of natural numbers
fn main() {
    let  num = 6;
    let mut sum: i32 = 0;
    for i in 1..num {
        sum += i;
    }

    println!("sum of first {} natural numbers is {}", num, sum);
}