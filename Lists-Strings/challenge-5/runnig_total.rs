fn main(){
    let numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    let runnig_total = get_running_total(&numbers);
    println!("{:?}", runnig_total)    
}

fn get_running_total(array:&[i32])->i32{
    let mut runnig_total = 0;
    for element in array.iter(){
        runnig_total +=  element 
    }
    runnig_total
}