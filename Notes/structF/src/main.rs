/*
    A structure is a user-defined data type that consists of variables of different data types.
    
    A structure is defined by using the struct keyword before the structure name. 
    
    Structure members are enclosed within the curly brackets. 
    
    Inside the curly brackets, structure members are defined with their name and type and structure 
    members are also known as fields.

*/
#[derive(Debug)]  //we have to use it print struct above the struct.
struct Employees{
    name: String,
    empid:i32,

}

struct Rectangle{
    length:i32,
    breath:i32,

}

/* 
-------------------------                             Update syntax                       --------------------------

if we want to update specific variable member then we use update keyword which is written as ".."and we can also update it by access 
specific data variable. 
*/

/*

-------------------------                            Method in Struct                      --------------------------

when the method is declared within the struct context, then the method syntax varies from the normal function. 

The first parameter of such methods is always "self", which represents the instance on which the function is called upon.

*/
impl Employees {

    fn name(&self)->i32{

        return self.empid;
    }
    
}

//  Here we use function in struct which is written like that ------

impl Rectangle{
    fn area(&self)->i32{
        return self.length*self.breath;
    }
}


fn main() {
    let x=Employees{
        name:String::from("aman"),
        empid:0,
    };
    let x1=Employees{           //using accessing specific data menber
        name:String::from("arslan"),
        empid :12,
    };
    let x2=Employees{           //using update syntax 
        name:String::from("aka"),
        ..x
    };
    let areaR=Rectangle
    {
        length:10,
        breath:30,
    };
    println!("{}",x.name);       //that's how we asscess any specific variable member of struct.
    println!("{:?}",x);
    println!("{:?}",x1);
    println!("{:?}",x2);
    println!("{}",x1.name());
    println!("{}",areaR.area())
    
}
