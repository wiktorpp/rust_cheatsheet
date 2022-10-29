fn main() {
    let array: [i32; 3] = [0, 1, 2];
    // println!("{:?}", array.get(2))
    let index = 2;
    match array.get(index) {
        Some(x) => println!("Number -> {:?}", x),
        None => println!("None!")
    }
}