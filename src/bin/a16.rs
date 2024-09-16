// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


struct Persons {
    age: i32,
    name: String,
    color: String,
}
fn print_data(name: &str, color: &str) {
    println!("Name: {}, Color: {}", name, color);
}
// * Use a struct for a persons age, name, and favorite color
fn main() {
    let my_people = vec![
        Persons {
            age: 20,
            name: String::from("Claire"),
            color: ("Gray").to_owned(),
        },
        Persons {
            age: 10,
            name: String::from("John"),
            color: ("Blue").to_owned(),
        },
        Persons {
            age: 5,
            name: String::from("Jane"),
            color: ("Green").to_owned(),
        },
    ];
    for people in my_people {
        if people.age <= 10 {
            print_data(&people.name, &people.color);
        }
    }
}