// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct Grocery {
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(grocery: &Grocery) {
    println!("Quantity: {:?}", grocery.quantity);
}

fn display_id(grocery: &Grocery) {
    println!("ID: {:?}", grocery.id)
}
fn main() {
    let grocery = Grocery {
        quantity: 10,
        id: 1,
    };
    display_quantity(&grocery);
    // * Create a function to display the id number, with the struct as a parameter
    display_id(&grocery);
}