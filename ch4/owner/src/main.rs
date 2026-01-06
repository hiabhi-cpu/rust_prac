fn main() {
    let s = "Hello" ;//this is a string literal 
    println!("{}",s);
    let mut s = String::from("Hello");//this is string literal from String which has multiple
                                      //functions to work on
    s.push_str("This is new");
    println!("Hello, world!{s}");
}
