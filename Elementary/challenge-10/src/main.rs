use chrono::prelude::*;


fn is_leap_year(year:i32)->bool{
    if ( year%4 == 0 || year%400 == 0 ) && year%100 !=0 {
        return true  
    }
    return false
}


fn main(){
    // get the current day
    let today: DateTime<Local> = Local::now();
    let today_year: i32 = today.year();

    println!("Your current year is {:?}, the following leap years are: ", today_year);

    for year in today_year..=today_year+20{
        if is_leap_year(year) {
            println!("{}", year)
        }    
    }

}