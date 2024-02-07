/*
    Enum is a custom data type which contains some definite values. It is defined with an enum keyword before the name of the
    enumeration. It also consists of methods.
*/
#[derive(Debug)] 
enum Gender{            //creation of enum Gender and where it is defined
    Male,
    Female,
}
#[derive(Debug)] 
struct Student{         //Here we define struct where enum get implemneted
    Name:String,
    Age:i64,
    Id:i32,
    Gender:Gender,
}
fn main() {
    let x=Student{
        Name:String::from("Aman"),
        Age:23,
        Id:34,
        Gender:Gender::Male,
    };
    println!("{:?}" ,x);
}