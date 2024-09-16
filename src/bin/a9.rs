// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Orange,
    Indigo,
}
#[allow(dead_code)]
fn main() {
    let my_color = Color::Blue;

    match my_color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
        Color::Purple => println!("purple"),
        Color::Orange => println!("orange"),
        Color::Indigo => println!("indigo"),
    }
}