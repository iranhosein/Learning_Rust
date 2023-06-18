fn main() {
    let msg = "Hosein Sharifi has good tutorials".to_string();
    let mut i = 1;
    
    for token in msg.split_whitespace() {
        println!("token {} {}", i, token);
        i+=1;
    }
}