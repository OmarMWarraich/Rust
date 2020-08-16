#[derive(Debug)]
struct Student {
    name: String,
    roll : u32
}
impl Student {
    fn printing(&self) { //method
        println!("Welcome in printing method");
        println!("Roll No of {} is {}", self.name, self.roll);
        println!("Complete User1 is {:#?}", self);
    }
    fn welcome(name:String,roll:u32) -> Student { //associated function
        println!("Welcome in WELCOME associated function");
        Student {
            name: name,
            roll: roll,
        }
        //u can do anything
    }
}

fn ppt(data:&Student){
    println!("Welcome in ppt function");
}
fn main() {
    let Student1 = Student {
        name: "Omar".to_string(),
        roll: 50152,
    };
    println!("Roll No. of {} is {}",Student1.name, Student1.roll);
    println!("Complete User 1 {:#?}",Student1);
    ppt(&Student1); //function calling
    Student1.printing(); //method calling
    let Student2= Student::welcome(String::from("Omar"),50152); //associated function calling
    Student2.printing(); 
}
