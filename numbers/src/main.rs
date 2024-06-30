fn main() {
    


    // change the variable type and arguments
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);


    // formating the type of the variable into another
    // let v: u16 = (38_u8 as i16).try_into().unwrap();

    // println!("Success! {}" , v);


    //basic of variables
    // let x = 5;
    // let mut y: u32 = 5;

    // y = x;
    
    // let z = 10; // Type of z ?  i32

    // println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
