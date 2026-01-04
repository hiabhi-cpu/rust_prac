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
    
    another_func(a[index]);
    
    let sum=add(2,3);

    println!("Sum is {sum}") ;

    if sum>10{
        println!("Sum is greater than 10");
    }else{
        println!("Sum is less than 10");
    }

    let check=if sum>10 {5} else {20};

    println!("Check:{check}");

    let mut count=0;

    let res = 'countingup: loop{
        count=count+1;

        if count == 10{
            break 'countingup count;
        }
    };
    println!("{res}");

    for num in (1..4).rev(){
        println!("{}",num);
    }
}

fn another_func(x: i32){
    println!("Hello {x}");
}

fn add(x: i32,y: i32) -> i32{
    return x+y;
}
