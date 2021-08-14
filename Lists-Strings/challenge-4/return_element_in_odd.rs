




fn main(){
    let numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    let odd_array = get_element_in_odd_pos(&numbers);
    println!("{:?}", odd_array)    
}

fn get_element_in_odd_pos(array:&[i32])->Vec<i32>{
    let range = array.len() - 1;
    let mut odd_array = Vec::new();
    for i in 0..=range {
        if i%2 != 0 {
           odd_array.push(array[i])
        }
    }
    odd_array
}