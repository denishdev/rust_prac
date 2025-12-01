struct Name{
    surname : String,
    first_name : String,
    age: u8,
}

fn main() {
    let name1 = Name{
        surname:String::from("den"),
        first_name:String::from("random"),
        age:30,
    };
    println!("{} {} {}",name1.surname,name1.first_name,name1.age);
}
