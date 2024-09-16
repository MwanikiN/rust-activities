// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Strawberry,
    Mango,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("Orange"),
        Flavor::Strawberry => println!("Strawberry"),
        Flavor::Mango => println!("Mango"),
    }
    println!("Ounces: {}", drink.ounces);
}

// * Use a match expression to print the drink flavor

fn main() {
    print_drink(Drink {
        flavor: Flavor::Orange,
        ounces: 12.0,
    });
}