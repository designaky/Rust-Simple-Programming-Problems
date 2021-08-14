use std::io::{stdin};

fn great(name:String){
    if name == "Alice" || name == "Bob" {
        println!("Hello, {}!", name);
    } else {
        println!("Hello!");
    }  
}

fn main(){
    let mut name = String::new();
    println!("Please enter your name :");
    stdin().read_line(&mut name).unwrap();
    great(name.trim().to_owned());
}