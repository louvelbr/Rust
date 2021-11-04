// ownership p.59
fn main() {
    //Variable scope
    {                // s is not valid here; it's not yet declared
    //thisis a literal
    let s = "hello";// s is valid from this point forward

    println!("s = {}", s); // do stuff with s
    }               // this scope is now over, and s is no longer valid

    //The String Type
    //let s = String::from("hello");
    //ou
    //this is a String
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s);

    // Memory and Allocation
    {                // s is not valid here; it's not yet declared
    let s = String::from("hello"); // s is valid from this point forward

    println!("s = {}", s); // do stuff with s
    }               // this scope is now over, and s is no longer valid

    //Ways That Variables and Data Interact: Move
    let s1 = String::from("hello");
    let s2 = s1; // s2 prend s1, s1 disparait

    println!("{}, world!", s1); //s1 n'existe plus

    //Ways That Variables and Data Interact: Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); //cette fois on copy s1 dans s2, les deux existent

    println!("s1 = {}, s2 = {}", s1, s2);

    //Stack-Only Data: copy
    let x = 5;
    let y = x; //both still exist because we know the size of integers

    println!("x = {}, y = {}", x, y);

}
