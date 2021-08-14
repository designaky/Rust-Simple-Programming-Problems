

fn main(){
    let numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];

    let max_value = largest(&numbers);
    
    println!("The biggest value in this list {:?} is {:?}", numbers, max_value)
}


fn largest(list: &[i32]) -> i32 {

    let mut max_value = list[0];
    for &item in list{
        if item > max_value {
            max_value = item;
        }    
    }
    max_value
}