fn main() {
    // calling a function
    fn_hello();
    println!("pi value is {}", get_pi());
    
    let no:i32 = 5;
    mutate_no_to_zero(no);
    println!("The value of no is: {}", no);
    
    let mut number:i32 = 5;
    mutate_no_to_zero_refrence(&mut number);
    println!("The value of no is: {}", number);
    
    let name:String = String::from("binhoosh");
    display(name);
    // cannot access name after display
}

// Defining a function
fn fn_hello() {
    println!("Hello from function fn_hello ");
}

fn get_pi()->f64 {
    22.0/7.0
}

fn mutate_no_to_zero(mut param_no:i32){
    param_no = param_no * 0;
    println!("param_no value is : {}", param_no);
}

fn mutate_no_to_zero_refrence(param_no:&mut i32){
    *param_no = 0; // de refrence
    println!("param_no value is : {}", param_no);
}

fn display(param_name:String){
    println!("param_name value is : {}", param_name);
}