
//normal addition in fuction
fn add1(){
    let a=45;
    let b=45;
    println!("{}",a+b);
}

// normal substraction function

fn subs(){
    let a=100;
    let b=50;
    println!("{}",a-b);
}

// normal multiplication function

fn mult(){
    let a=32;
    let b=10;
    println!("{}",a*b);
}

//parametric funtion use with return statement(addition)
fn add( par:i64)->i64{
    let a=23;
    return a + par;
}

//parametric funtion use with return statement(substraction)

fn subs1(par:i64)->i64{
    let a=1000;
    return a - par;
}

//parametric funtion use with return statement(multiplication)

fn mult1(par:i64)->i64{
    let a=32;
    return a * par;
}

//parametric funtion use with return statement(division)

fn div(){
    let a=230;
    let b=10;
    println!("{}",a/b);
}

//parametric funtion use with return statement(division)

fn div1(par:i64)->i64{
    let a = 1000;
    return a/par;
}


fn main() {
    let par:i64=100;

    add1();
    subs();
    mult();
    div();

    let res=add(par);
    println!("{}",res);

    let res1=subs1(par);
    println!("{}",res1);

    let res2=mult1(par);
    println!("{}",res2);
    
    let res3=div1(par);
    println!("{}",res3);
   
}
