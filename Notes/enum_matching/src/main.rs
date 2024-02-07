/*  

-----------------------                       Match Operator                    -----------------------
      
The match operator allows us to compare a value against a series of patterns, and executes the code whenever the match is found. 

The patterns can be literal values, variable names, wildcards and many other things.

*/
enum Square{
    Length,
    Breath,
}
fn area(area:Square){
    match area{
        Square::Length=>println!("l"),
        Square::Breath=>println!("b"),
    }
}

fn main() {
 
    area(Square::Length);
    area(Square::Breath);
}
