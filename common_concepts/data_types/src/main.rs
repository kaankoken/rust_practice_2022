/**
 * Data types split into scalar and compound types.
 * Scalar types are fundamental data types that are not composed of other data types.
 * Such as integers, floats, characters, booleans, etc.
 * 
 * Compound types are data types that are composed of other data types.
 * In rust, compound types are tuples, arrays, and slices.
 * Tuples are immutable and are represented by ()
 * Arrays are mutable and are represented by []
 * Slices are immutable and are represented by &[T]
 * 
 */
fn main() {
    // Example of a unsigned 8 bit integer
    let x: u8 = 123;
    println!("The value of x is: {}", x);

    // Example of a signed 8 bit integer
    let y: i8 = -123;
    println!("The value of y is: {}", y);

    let guess: u16 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
    
    // Example of a float
    let z: f32 = 2.5;
    println!("The value of z is: {}", z);

    // Example of a boolean
    let a: bool = true;
    println!("The value of a is: {}", a);

    // Example of a character
    let b: char = 'a';
    println!("The value of b is: {}", b);

    // isize and usize are signed and unsigned integers respectively
    // isize is usually for offset of memory addresses, positions, indices or length.
    // usize is usually for memory addresses, positions, indices or length.
    // usize is usually used for array sizes
}
