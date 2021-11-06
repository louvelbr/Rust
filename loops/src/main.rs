//Repetition with Loops p.54


fn main() {
    //Repeating Code with loop
    /*loop{
        println!("again!");
    }*/

    //Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    //Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    //Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];

    //avec while (pas ouf)
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    //avec for
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    //countdown with for and reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
