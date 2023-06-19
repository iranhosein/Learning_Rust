#[derive(Debug)]
enum GenderCategory {
    Male, Female
}

#[derive(Debug)]
struct Person {
    name:String,
    gender:GenderCategory
}

#[derive(Debug)]
enum GenderCategory2 {
    Name(String), UsrId(i32)
}

enum CarType {
    Hatch,
    Sedan,
    SUV
}

fn main() {
    
    let p1 = Person {
        name:String::from("Hosein"),
        gender:GenderCategory::Male
    };
    
    let p2 = Person {
        name:String::from("Zahra"),
        gender:GenderCategory::Female
    };
    
    let male = GenderCategory::Male;
    let female = GenderCategory::Female;
    
    let result = is_even(3);
    
    let p3 = GenderCategory2::Name(String::from("Ali"));
    let p4 = GenderCategory2::UsrId(100);
    
    println!("{:?}", male);
    println!("{:?}", female);
    
    println!();
    
    println!("{:?}", p1);
    println!("{:?}", p2);
    
    println!();
    println!("Employee 1:");
    println!("{:?}, {:?}", p1.name, p1.gender);
    
    println!();
    
    println!("{:?}", result);
    println!("{:?}", is_even(30));
    
    println!();
    
    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
    
    println!();
    
    match is_even(5) {
        Some(data) => {
            if data == true {
                println!("Even no");
            }
        },
        None => {
            println!("not even");
        }
    }
    
    println!();
    
    println!("{:?}", p3);
    println!("{:?}", p4);
    
    match p3 {
        GenderCategory2::Name(val) => {
            println!("{}", val);
        }
        GenderCategory2::UsrId(val) => {
            println!("{}", val);
        }
    }
}

fn is_even(number:i32)->Option<bool> {
    if number%2 == 0 {
        Some(true)
    } else {
        None
    }
}

fn print_size(car:CarType) {
    match car {
        CarType::Hatch => {
            println!("Small sized car");
        },
        CarType::Sedan => {
            println!("medium sized car");
        },
        CarType::SUV => {
            println!("Large sized Sport Utility Car");
        }
    }
}