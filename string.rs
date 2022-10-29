fn main() {
    let my_str = "hello"; // immutable
    let my_string = String::from(my_str); // mutable
    let my_string = my_str.to_string();   // same thing

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
}