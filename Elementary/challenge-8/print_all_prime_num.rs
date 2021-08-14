fn is_prime(num:i64)->bool{
    if num <= 1 {
        return false
    }
    for i in 2..=num-1 {
        if num%i == 0 {
            return false
        }
    }  
    return true;
    
}

fn main(){
    const UPPER :i64 = i64::MAX;
    for num in 2..=UPPER{
        if is_prime(num) {
            print!("{} ", num )
        }
    }
}