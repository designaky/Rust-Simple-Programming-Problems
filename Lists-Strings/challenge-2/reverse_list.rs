

fn main(){
    let numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    let max_value = reverse_array(&numbers);
    println!("{:?}", max_value)
}

fn reverse_array(array:&[i32])->Vec<i32>{
    array.iter().copied().rev().collect()
}
