/* because variables are immutable in rust use

mut - keyword we can use to set the x variable to be mutable, thus we can then allow x to be set again, in x = 5
*/


fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
