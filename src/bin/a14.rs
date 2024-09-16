// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
#[derive(Debug)]
enum Color {
    Brown,
    Black, 
}
#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

// * Implement functionality on the box struct to print the characteristics
impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

// * Implement functionality on the box struct to create a new box
impl Box {
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Box {
            dimensions,
            weight,
            color,
        }
    }
    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        println!("dimensions: {:?}", self.dimensions.print());
        println!("weight: {:?}", self.weight);
        println!("color: {:?}", self.color);
    }
}

// * Implement functionality on the box struct to print the characteristics

fn main() {
    let dimensions = Dimensions {
        width: 10.0,
        height: 10.0,
        depth: 10.0,
    };
    let box1 = Box::new(dimensions, 10.0, Color::Brown);
    box1.print();
    // * Use an enum for the box color
}