


fn main(){
    let numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    let element = 2;

    // if find_element(&numbers, &element) {
    //     print!("The ")
    // }

    println!("{:?}", find_element(&numbers, element))
}


fn find_element(list:&[i32], element:i32)->bool{
    for &item in list{
        if item == element {
           return true
        }    
    }
    return false
}
