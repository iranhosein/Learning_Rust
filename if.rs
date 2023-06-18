fn main() {
    let number:i32 = 5;
    if number > 0 {
        println!("number is positive");
        if number%2==0 {
            println!("Even Number");
        } else {
            println!("Odd Number");
        }
    } else if number == 0 {
        println!("number is 0");
    } else {
        println!("number is negetive");
    }
}