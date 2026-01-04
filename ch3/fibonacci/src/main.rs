use std::io;

fn main() {
    println!("Hello, world!");
    
    println!("Enter the number till which u need the fibonaci: ");

    let mut n=String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read");

    let n:i32 = n.trim()
                    .parse()
                    .expect("Error in converting to int");

    println!("{}",n);

    let mut a=0;
    let mut b=1;

    for _ in (0..n).rev(){
        let c=a+b;
        a=b;
        b=c;
        println!("{}",c);
    }
}
