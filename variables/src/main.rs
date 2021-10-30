/*Chapter 3: common programming concepts p.31*/
fn main() {
    /*mutable variables*/
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    /*Immutable variables, shadowing*/
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is {}", y); //y = 12

    let spaces = "   ";
    let spaces = spaces.len(); //permet de changer le type d'une variable

    println!("The value of spaces is {}", spaces);

                    /*DATA TYPES*/

    /*Numeric operations p.39 (=> list in Appendix B)*/
    //addition
    let sum = 5+10;

    //substraction
    let difference = 95.5 - 4.3;

    //division
    let quotient = 56.7 / 32.2;

    //remainder
    let remainder = 43 % 5;

    /*Boolean Type*/
    let t = true;
    let f: bool = false; // with explicit type annotation

    /*Character Type*/
    let c = 'z'; //single quote
    let z = 'Z';
    let heart_eyed_cat = '😻';

                /*Compound Types p.40*/
    /*Tuple Type*/
    let tup: (i32, f64, u8) = (500, 6.4, 1); //different types

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    /*Array Type*/ //fix size
    let a = [1, 2, 3, 4, 5]; //same types

    let months = ["January", "February", "March", "April", "May"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];  //i32: type and 5: size

    let a = [3; 5]; // <=> let a = [3, 3, 3, 3, 3, 3];

    /*Accessing Array Elements*/
    let a = [1, 2, 3, 4, 5];
    let first = a[0]; //first = 1

}
