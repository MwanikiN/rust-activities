// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}
fn main() {
    let my_student = Student {
        name: "Jane".to_owned(),
        locker: Some(20),
    };
    println!("Name: {:?} ", my_student.name);
    match my_student.locker {
        Some(num) => println!("Locker: {:?}", num),
        _ => println!("No locker assigned"),
    }
    
    }
    
