fn main() {
    let arr:[i32; 4] = [10, 20, 30, 40];
    println!("array is {:?}", arr);
    println!("array size is : {}", arr.len());
    
    println!();
    
    let arr2 = [10, 20, 30, 40];
    println!("array2 is {:?}", arr2);
    println!("array2 size is : {}", arr2.len());
    
    println!();
    
    let mut arr3:[i32;4] = [-1;4];
    println!("array3 is {:?}", arr3);
    println!("array3 size is : {}", arr3.len());
    
    println!();
    
    for index in 0..4 {
        println!("index is: {} & value is : {}", index, arr[index]);
    }
    
    println!();
    
    for val in arr.iter(){
        println!("value is : {}", val);
    }
    
    println!();
    
    let mut arr4:[i32;4] = [10, 20, 30, 40];
    arr4[1] = 0;
    println!("{:?}", arr);
    
    println!();
    
    update(arr);
    println!("Inside main {:?}", arr);
    
    println!();
    
    update_reference(&mut arr3);
    println!("Inside main {:?}", arr);
}

fn update_reference(arr:&mut [i32;4]){
    for i in 0..4 {
        arr[i] = 12;
    }
    println!("Inside update by reference {:?}", arr);
}

fn update(mut arr:[i32;4]){
    for i in 0..4 {
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}