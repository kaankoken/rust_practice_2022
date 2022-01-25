/**
 * By default all defined variables are immutable 
 */

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // (Line 9) It gives an error before compile time 
    x = 6;
    println!("The value of x is: {}", x);

    // Fixed version
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);
}
