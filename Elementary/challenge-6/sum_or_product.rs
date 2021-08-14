use std::io::{stdin};

fn sum(num1:i32, num2:i32)->i32{
    num1 + num2
}

fn product(num1:i32, num2:i32)->i32{
    num1 * num2
}


fn get_input(message:&str) -> String {
    print!("{}", {message});
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed");
    input.trim().parse::<String>().unwrap()
}

fn convert_to_num(input:String)->i32{
    let num = input.parse::<i32>().unwrap();
    num
}

fn main(){
    let num1 = convert_to_num(get_input("Please insert your number:\n"));
    let num2 = convert_to_num(get_input("Please insert your number:\n"));
    let operator = get_input("Write: \nsum: for the sum \nprod: for the product\n");

    match operator.as_str() { 
        "sum" => print!("The sum is equal: {}\n", sum(num1, num2)),
        "prod" => print!("The prod is equal: {}\n", product(num1, num2)),
        _ => println!("Please chose between sum and prod"),
    }
}