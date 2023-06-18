fn main(){
    let name1 = "binhoosh".to_string();
    let name2 = "iranhosein".to_string();
    
    let name1_copy = name1.to_string();
    let name3 = name1_copy + &name2;
    let name2_copy = name2.to_string();
    let name4 = name2_copy + &name1;
    
    println!("1- {}", name1);
    println!("2- {}", name2);
    println!("3- {}", &name3);
    println!("4- {}", name3);
    println!("5- {}", name4);
}