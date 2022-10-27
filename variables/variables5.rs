/*
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
did shadowing, basically i set the value of the number variable again by putting it inside a new parenthesis to create a new scope block and set the number variable again with the let keyword
 */

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    {
        println!("Spell a Number : {}", number);
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
}
