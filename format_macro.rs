fn main() {
    let name1 = "binhoosh".to_string();
    let name2 = "iranhosein".to_string();
    let name3 = format!("{} {}", name1, name2);
    println!("{}", name3);
    
    println!();
    
    for token in name3.split(" "){
        println!("{}", token);
    } 
}