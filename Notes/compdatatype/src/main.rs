//Compound datatype are those which can store more than one value at a time and it can be of 
//different types.

//Basically there are two type of compound datatypes 1.Tuple and 2.Array.


fn main() {

    tup();
    arr();
    }

/*  
        Tuple is a compound data type. A scalar type can store only one type of data.

        For example, an i32 variable can store only a single integer value. 

        In compound types, we can store more than one value at a time and it can be of different types.

        Tuples have a fixed length - once declared they cannot grow or shrink in size. 

        The tuple index starts from 0.
 */

 fn tup(){
    let tup1=(1,2,true,'Y');   //type 1 of tuple intialization 
    let tup2:(i32,bool,char)=(12,true,'N');            //type 2 of tuple intialization
    println!("{:?}",tup1);
    println!("{}",tup2.2);
}


/*  
            -----------------     Features of an Array        --------------

        An array declaration allocates sequential memory blocks.

        Arrays are static. This means that an array once initialized cannot be resized.

        Each memory block represents an array element.

        Array elements are identified by a unique integer called the subscript/ index of the element.

        Populating the array elements is known as array initialization.

        Array element values can be updated or modified but cannot be deleted. */


fn arr(){
    let arr1=[1,2,3,4,5];        //type 1 of array intialization
    let arr2: [i32;4]=[1,2,3,4];           //type 2 of array intialization
    let arr3:[i32;5]=[5;5];                //type 3 of array intialization
    println!("{:?}",arr1);
    println!("{:?}",arr2);               
    println!("{:?}",arr3);
}
