// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust"; // unchangeable
const THRESHOLD: i32 = 10;

fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Explicit conversion
    let integer = an_integer as u8;
    let character = integer as char;
}
