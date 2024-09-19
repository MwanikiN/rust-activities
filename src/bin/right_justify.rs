fn main() {
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);
    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=6);
}