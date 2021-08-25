fn main(){
    let numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    let sum_for = get_sum_for(&numbers);
    let sum_while = get_sum_while(&numbers);
    let sum_recursion = get_sum_recursion(&numbers);
    println!("For sum {:?}", sum_for);  
    println!("While sum {:?}", sum_while);  
    println!("Recursion sum {:?}", sum_recursion);  
}

fn get_sum_for(array:&[i32])->i32{
    let mut sum = 0;
    for element in array.iter(){
        sum +=  element 
    }
    sum
}


fn get_sum_while(array:&[i32])->i32{
    if array.len() == 0 {
        return 0
    }
    let mut sum = 0;
    let mut index = 0;
    let last_index = array.len() - 1;
    while index <= last_index {
        sum += array[index];
        index += 1;
    }
    sum
}

fn get_sum_recursion(array:&[i32])->i32{
    if array.len() == 0 {
        print!{"{:?}", array.len() == 0}
        return 0
    }

    if array.len() == 1 {
        return array[0]
    } 

    return array[0] + get_sum_recursion(&array[1..])
}

#[test]
fn test_get_sum_for() {
    test_sum_base(get_sum_for);
}

#[test]
fn test_get_sum_while() {
    test_sum_base(get_sum_while);
}

#[test]
fn test_get_sum_recursion() {
    test_sum_base(get_sum_recursion);
}


#[cfg(test)]
fn test_sum_base<F>(sum: F) where F: Fn(&[i32]) -> i32 {

    let numbers_0 = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    assert_eq!(sum(&numbers_0),1007);

    let numbers_1 = vec![-1,-3,-4,-2];
    assert_eq!(sum(&numbers_1), -10);

    let numbers_3 = vec![];
    assert_eq!(sum(&numbers_3), 0);

    let numbers_4 = vec![0];
    assert_eq!(sum(&numbers_4), 0);

}