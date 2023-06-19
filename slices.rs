fn main() {
    let name1 = "binhoosh".to_string();
    println!("length of string is {}", name1.len());
    let c1 = &name1[3..8];
    
    println!("{}", c1);
    
    println!();
    
    let data = [10, 20, 30, 40, 50];
    use_slice(&data[1..4]);
    
    println!();
    
    let mut data2 = [10, 20, 30, 40, 50];
    use_slice2(&mut data2[1..4]);
    println!("{:?}", data2);
}

fn use_slice(slice:&[i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
}

fn use_slice2(slice:&mut [i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
    slice[0] = 57;
}