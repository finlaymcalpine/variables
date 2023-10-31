fn main() {
    // we need to add mut in fornt of the variable name to allow mutability.
    // variables immutable by default.
    // another method could be to shadow x by writing let x = 6, which wouldn't error.
    let mut x: i32 = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let x: &str = "banana";
    println!("The value of x is: {x}");
    // We can't mutate a variable's type, so using the same name for a different type 
    // requires shadowing.

    // char variables have to be declared in single quotes
    let c: char = 't';

    // floats (either f32 or f64) are pretty standard, I think
    let f: f64 = 4.0;

    // Tuples are a compound of different type values: they cannot made larger/smaller after creation
    // 
    let tup: (i32, f64, char) = (100, 5.2, 'h');
    let one_hundred = tup.0;

    // Arrays are a compound of THE SAME type for all values. Rust arrays also have a fixed length.
    // If looking to expand/shrink, then use a Vector type. So use arrays for unchanging groupings.
    // Arrays are also allocated on the stack, rather than the heap.
    // Accessed with square bracket indexing.
    // Runtime check results in error if trying to access an array element past the index.
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let zeroes = [0, 5];
    let first_elem = zeroes[0];

}