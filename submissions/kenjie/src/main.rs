// Dominic and Kenjie
fn main() {
    let mut num = 12;
    let mut str = String::from("21");

    println!("{}", string_to_integer(&mut str));
    println!("{}", integer_to_string(&mut num));
}

fn string_to_integer(str: &mut String) -> i32 {
    str.to_string().parse::<i32>().unwrap()
}

fn integer_to_string(num: &mut i32) -> String {
    num.to_string()
}
