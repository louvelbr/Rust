fn main() {
    println!("Hello, world!");

    another_function();

    another_functionbis(5, 6);

    //Statements and Expressions in Functions Bodies
    /*let x = 5;

    let y = {
        let x = 3;
        x + 1;
    };
    println!("The value of y is: {}", y);*/

    let x = five(); // = let x = 5;
    println!("The value of x is: {}", x);

    let x2 = plus_one(5);
    println!("The value of x2 is: {}", x2);

}

fn another_function() {
    println!("Another function.");
}

//Functions Parameters
fn another_functionbis(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

//Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
