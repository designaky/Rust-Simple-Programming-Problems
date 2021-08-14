


fn print_line(){
    let mut line = "".to_owned();
    for _num in 0..=34 { 
        let borrowed_string: &str = "__";
        line = format!("{}{}", borrowed_string, line);
    }
    println!("{}\n", line);
}

fn get_space(num:&i32)->String{
    let space;
      if num < &10 {
        space = "   ".to_string()
       } else if num < &100 {
        space = "  ".to_string()
       } else {
        space = " ".to_string()
       }
    space
}

fn print_number_line(number_vec:&Vec<i32>, line_num:i32){
    let mut line = "".to_owned();
    for num in number_vec.iter() { 
        line = format!("{}{}{:?}|", line, get_space(num),num);  
    }  
    println!("{}{:?}|{}", get_space(&line_num),line_num,line);
    print_line();
       
}



fn main(){  
    let max = 12;  
    let number_array: Vec<i32> = (0..max+1).collect();
    print_line(); 
    print_number_line(&number_array, 0);
    for num in 0..=max { 
        let new_array: Vec<_> = number_array.iter().map(|x| x * num).collect();
        print_number_line(&new_array, num);      
    }

}