struct Employee {
    name: String,
    company: String,
    age: u32
}

struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self)->u32 {
        self.width * self.height
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_instance(x:i32, y:i32) -> Point {
        Point {x:x, y:y}
    }
    fn display(&self) {
        println!("x={} y={}", self.x, self.y);
    }
}
    
fn main() {
    let emp1 = Employee {
        company:String::from("binhoosh"),
        name:String::from("iranhosein"),
        age: 50
    };
    
    let mut emp2 = Employee {
        company:String::from("bina"),
        name:String::from("reza"),
        age: 35
    };
    emp2.age = 45;
    
    let emp3 = Employee {
        company:String::from("Microsoft"),
        name:String::from("Mohammad"),
        age:32
    };
    
    let emp4 = Employee {
        company:String::from("Apple"),
        name:String::from("Ali"),
        age:28
    };
    
    let emp5 = Employee {
        company:String::from("Amazon"),
        name:String::from("Arash"),
        age: 37
    };
    
    let elder = who_is_elder(emp4, emp5);
    
    let small = Rectangle {
        width: 10,
        height: 20
    };
    
    let p1 = Point::get_instance(10, 20);
    
    println!("Emp1 =>");
    println!("Name is : {}\ncompany is {}\nage is {}", emp1.name, emp1.company, emp1.age);
    println!();
    println!("Emp2 =>");
    println!("Name is : {}\ncompany is {}\nage is {}", emp2.name, emp2.company, emp2.age);
    println!();
    println!("Emp3 =>");
    println!("Name is : {}\ncompany is {}\nage is {}", emp3.name, emp3.company, emp3.age);
    println!();
    display(emp1);
    display(emp2);
    println!();
    println!("elder is: ");
    display(elder);
    println!();
    println!("width is {}\nheight is {}\narea of rectangle is {}", small.width, small.height, small.area());
    println!();
    p1.display();
}

fn display(emp:Employee) {
    println!("Name is : {}\nCompany is {}\nAge is {}", emp.name, emp.company, emp.age)
}

fn who_is_elder(emp1:Employee, emp2:Employee)->Employee {
    if emp1.age>emp2.age {
        return emp1;
    } else {
        return emp2;
    }
}