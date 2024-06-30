fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");


    //destruction of variables can be done in many ways here the mut is not assignable to a group of variables instead we have to add it individually
    // let (mut x, y) = (1, 2);
    // x += 2;

    // assert_eq!(x, 3);
    // assert_eq!(y, 2);

    // println!("Success!");


    // Basics of shadwing is the variable is declared at first can be redifined later in different form like mutable and immutable variables so we can shadow that variable and rebuild them as needed
    // let mut x: i32 = 1;
    // x = 7;
    // // Shadowing and re-binding
    // // let x = x; 
    // x += 3;


    // let y = 4;
    // // Shadowing
    // let y = "I can also be bound to text!"; 

    // println!("Success!");


    // To check the assert_eq functionhow it worked and basics of it
    // let x: i32 = 5;
    // {
    //     let x = 12;
    //     assert_eq!(x, 12);
    // }

    // assert_eq!(x, 5);

    // let x = 42;
    // println!("{}", x); // Prints "42".

    // How a function run and compiles 
    // let x = define_x();
    // println!("{}, world", x); 
}

fn define_x() -> &'static str {
    "hello"
}