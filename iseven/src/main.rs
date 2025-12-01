fn main() {
    let x = 10;
    println!("Is even {}", is_even(x));
    fibunachi(x);
}

fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    return false;
}

fn fibunachi(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if n == 0 {
        return first;
    }

    if n == 1 {
        return second;
    }
    println!("{}", first);
    println!("{}", second);
    for _i in 2..n {
        let next = first + second;
        first = second;
        second = next;
        println!("{}", second);
    }
   
    return second;
}
