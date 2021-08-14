use std::io::{stdin};

fn sum(num:i32)->i32{
    num + 1
}

fn get_input() -> String {
    print!("Please insert your number:\n");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn convert_to_num(input:String)->i32{
    let num = input.trim().parse::<i32>().unwrap();
    num
}

fn main(){
    let num = convert_to_num(get_input());

    if num % 3 == 0 || num % 5 == 0 {
        print!("The sum is equal: {}\n", sum(num))
    }
    
}
