fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of s is: {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }

    println!("The value of x is: {x}");

    let spaces = "  ";
    let spaces = spaces.len();
    println!("The spaces is {spaces}");

    // float data type
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3;

    let _remainder = 43 % 5;
}
