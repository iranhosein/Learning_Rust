fn main() {
    let v = vec![10, 20, 30];
    print_vector(&v);
    println!("printing the value from main() v[0]={}", v[0]);
    
    println!();
    
    let mut i = 3;
    add_one(&mut i);
    println!("New number is : {}", i);
    
    println!();
    
    let mut name:String = String::from("binhoosh");
    display(&mut name);
    println!("The value of name after modification is : {}", name);
}

fn display(param_name:&mut String) {
    println!("param_name value is : {}", param_name);
    param_name.push_str(" Rocks");
}

fn print_vector(x:&Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}

fn add_one(e: &mut i32) {
    *e+=1;
}