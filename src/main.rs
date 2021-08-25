fn main(){
    let numbers: vec![i32] = vec![4, 65, 2, 0, 99, 2, 83, 782, 1];
    let sum_for = on_all<i32>(&numbers, sum);

}

fn on_all<T>(array:&[T], function:fn<Y>(Y)->Y)->vec![T]{

[23]

}

fn sum(num: i32)-> i32{
    num + 1
}