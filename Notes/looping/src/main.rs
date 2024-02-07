fn main() {
    let x=1;
    println!("LOOPING STATEMENTS -------------");
    println!();
    first(x);
    second(x);
    third();
}
//There are threee ways to perform loop iteration

//1.using Loop------------------------

fn first(mut x:i32){

    println!("Performed using loop iteration");
    println!();
    loop{
        println!("The number is {}",x);
        x+=1;
        if x ==15{
            break;
        }
    }

}

//2.Using While Loop------------------

fn second(mut x :i32){
    println!();
    println!("Performed using while iteration");
    println!();
    while x !=15{
        println!("The number is {}",x);
        x+=1;
    }
}

//3.Using For loop---------------------

fn third(){
    println!();
    println!("Performed using for iteration");
    println!();
    for x in 1..15{
        println!("The number is {}",x);
    }

}