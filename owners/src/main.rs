fn main() {
    // println!("Hello, world!");
    // let s1 = String::from("i'm owner");
    // let s2 = s1;
    // println!("{}",s2);

    //Reference

    let mut s1 = String::from("Hello");
    append_str(&mut s1);
    println!("{}",s1);
}

fn append_str(s: &mut String){
    s.push_str("world");
}