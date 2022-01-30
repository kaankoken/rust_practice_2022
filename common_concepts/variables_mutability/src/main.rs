/**
 * By default all defined variables are immutable 
 */

fn main() {
    /*
        let x = 5;
        println!("The value of x is: {}", x);
        // (Line 9) It gives an error before compile time 
        x = 6;
        println!("The value of x is: {}", x);
    */

    // Fixed version of the example above
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    constants_and_shadowing();
}

// Function for contants & shadowing example
fn constants_and_shadowing() {
    // Constants are immutable by default and cannot use mut keyword

    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing example
    let x = 5;
    let x = x + 1;
  
    {
        let x = x * 2;
        println!("The value of inner x is: {}", x);
    }

    println!("The value of x is: {}", x);

    // Shadowing example
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    /*
        // Shadowing example
        let mut spaces = "   ";
        // It gives an error before compile time since space is mutable & the value type is string
        spaces = spaces.len();

        println!("The value of spaces is: {}", spaces);
    */
}