
use rand::{thread_rng, Rng};
use std::mem::{MaybeUninit};
use std::io::{stdin};


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

fn check_guess(user_guess:i32, secret_number:i32)->(String,bool){
    if user_guess == secret_number {
        let message: String = format!("Congratulation your {} was the right one, what a wiz!!", user_guess);
        return (message,true) ;
    } else if user_guess < secret_number{
        let message: String = format!("Your {} guess is low.", user_guess);
        return (message,false) ;
    } else {
        let message: String = format!("Your {} guess is high.", user_guess);
        return (message,false) ;
    } 
}



fn main(){
    let mut win = false;
    let mut available_guesses: i32 = 3; 
    let mut old_user_guess: i32 = unsafe {  MaybeUninit::uninit().assume_init() }; 
    let secret_number: i32 = thread_rng().gen_range(0..100);
        
    println!("{}",secret_number);
    println!("Welcome to guess the number, you have {} attempts to guess the right number", available_guesses);
    while available_guesses > 0 && win == false  {
        let user_guess = convert_to_num(get_input("Please insert your guess:\n"));

        if old_user_guess == user_guess {
            println!("Your previous guess was the same as your current {}, change your guess.", user_guess);      
        } else {
            available_guesses-=1;
            old_user_guess = user_guess;
            let result: (String,bool) = check_guess(user_guess, secret_number);
            println!("{}",result.0);
            win = result.1;
            if win == false && available_guesses == 0 {
                print!("Sorry you finished your guesses you lose!\n");
            }  
        }
        
    }
}