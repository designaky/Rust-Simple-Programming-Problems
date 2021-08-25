use std::io::{stdin};

fn is_palindrome(string: &String)->bool{
    if string.len() == 0 {
        return false
    }
    let trim_string = string.trim_end();
    let mut index = 0;
    let mut reverse_index = trim_string.len() - 1;
    let maybe_palindrome: Vec<char> = trim_string.chars().collect();
    while index < reverse_index {
        if maybe_palindrome[index].to_ascii_lowercase() != maybe_palindrome[reverse_index].to_ascii_lowercase(){
            return false
        }
        index += 1;
        reverse_index -= 1;
    }

    true

}

fn main(){
    let mut string = String::new();
    println!("Please enter your string: ");
    stdin().read_line(&mut string).unwrap();
    if is_palindrome(&string) {
        println!("The string is a palindrome")
    } else {
        println!("The word is not a palindrome")
    }
  
}


#[test]
fn test_is_palindrome() {
    print!("Hello test 1")
    assert!(is_palindrome(""));
    assert!(is_palindrome("a"));
    assert!(is_palindrome(".,,/.,/.,././.,=-=--"));
    assert!(is_palindrome("a,,,,,,,,,,,,,,,a"));
    assert!(is_palindrome("a,,,,,bb,,,,,,,,a"));
    assert!(is_palindrome("race car"));
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("raccar"));
}


