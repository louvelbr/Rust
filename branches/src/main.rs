//if Expressions p.49
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Handling Multiple Conditions with else if
    let numberbis = 6;

    if numberbis % 4 == 0 {
        println!("number is divisible by 4");
    } else if numberbis % 3 == 0 {
        println!("number is divisible by 3");
    } else if numberbis % 2 == 0 {
        println!("number is divisible by 2");
    } else{
        println!("number is not divisible by 4, 3, or 2");
    }

    //Using if in a let Statements
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}",number);
}
