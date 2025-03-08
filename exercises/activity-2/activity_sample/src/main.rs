fn main() {
    let first_character = 'L';
    let mut name = String::from("Sophia");
    name = first_character.to_string() + &name[1..];
    println!("{}", name);
}
