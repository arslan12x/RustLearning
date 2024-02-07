/*
----------------------                    Matching with Option<T>                      -----------------------

    Option<T> is used when we want to get the inner value of T out of some case.

    The Option<T> consists of two variants:

       1.None: It indicates the failure or lack of value.
       2.Some(value): It is a tuple struct that wraps the value with T.

*/
fn number(num:i32)->Option<bool> {
    if num%2==0{
        Some(true)
    }
    else{
        None
    }
}

fn numm(num:i32){

    match check(num) {
        Some(num) =>{
            if num%2==0{
                println!("even");
            }
            else{
                println!("odd");
            }
        }
        None => println!("none"),
}}

fn check(number:i32)->Option<i32>  
{      
  Some(number%2)  
}

fn main() {
    numm(567);
    let x =number(44);
    println!("{:?}",x);
}