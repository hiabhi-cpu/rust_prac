use std::io;
    const ABHI: u32 = 2;

fn main() {
    let x = 10;
    println!("before x {x}");
    
    let x=100;
    println!("after x {x}");
    
    
    println!("Constant {ABHI}");

    {
        let x="hello";
        println!("x value inside a different block {x}");
    }

    println!("After block x {x}");

    let var:u8 = 255;

    println!("var {var}");
    let var = 56.3/32.2; 
    println!("{var}");

    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    let third= tup.2;
    println!("The third values is {third}");

    let arr=[1,2];

    let first=arr[0];

    println!("Array {first} ");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
