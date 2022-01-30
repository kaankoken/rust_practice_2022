fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    println!("The value of method is {}", test_method_with_return());
}

fn test_method_with_return() -> i8 {
    return 16;
}
