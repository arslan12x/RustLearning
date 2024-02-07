
//Ownership is the unique feature that Rust programming language offers and providesthe guarantee of memory safety without using garbage 
//collector or pointers.

/*
-----------------------------------  Rules of Ownership-------------------------------

                In Rust, every value has a variable associated with it and that is called its owner.
                There can only be one owner at a time.
                When the owner goes out of scope, the value associated with it is destroyed.

*/

fn main() {
    own();

}
//If we have to copy datatype in stack memory then we only have to assign it and when we have to do it heap memory then we use clone.
fn own(){
    let x:&str="hey";
    let y=x;
    let z=x;
    let x1=String::from("hey");
    let x2=x1.clone();
    println!("{x}----{y}-----{z}----{x1}----{x2}");

}
