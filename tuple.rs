fn main() {
    let tuple:(i32, f64, u8) = (-325, 4.9, 22);
    println!("{:?}", tuple);
    println!("integer is : {:?}", tuple.0);
    println!("float is : {:?}", tuple.1);
    println!("unsigned integer is : {:?}", tuple.2);
    
    println!();
    
    let b:(i32, bool, f64) = (110, true, 10.9);
    print(b);
    
    println!();
    
    let c:(i32, bool, f64) = (30, true, 7.9);
    print2(c);
}

// pass the tuple as a parameter
fn print(x:(i32, bool, f64)){
    println!("Inside print method:");
    println!("{:?}", x);
}

fn print2(x:(i32, bool, f64)){
    println!("Inside print2 method:");
    let (age, is_male, cgpa) = x; // assigns a tuple to distinct variabels
    println!("Age is {}\nisMale? {}\ncgpa is {}", age, is_male, cgpa)
}