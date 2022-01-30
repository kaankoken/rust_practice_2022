/**
 *
 * Data types split into scalar and compound types.
 * Scalar types are fundamental data types that are not composed of other data types.
 * Such as integers, floats, characters, booleans, etc.
 *
 * Compound types are data types that are composed of other data types.
 * In rust, compound types are tuples, arrays, and slices.
 * Tuples are immutable and are represented by ()
 * Arrays are mutable and are represented by [
 * Unlike other languages, rust does not have a fixed size array.
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

    // Example of a tuple
    let tup = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let (x, y, z): (i32, f32, u8) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // Example of an array
    // To display array or tuple, use {:?}
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    let months = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    println!("The value of months is: {:?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    // First index indicates the number and second index indicates the size of the array
    let a = [3; 5];
    println!("The value of a is: {:?}", a);

    print!("Enter an index number: ");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input.trim().parse().expect("Not a number!");

    let element = a[input];

    println!(
        "The value of the element at index {} is: {}",
        input, element
    );
}
