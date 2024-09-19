fn main(){
    let num = 9;
    match num {
        num if num % 2 == 0 => println!("Even"),
        _ => println!("Odd")
    }
}