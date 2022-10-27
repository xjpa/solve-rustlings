/*
read about constants and variable's mutability https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants

just annotate the NUMBER variable as constants must always be annotated
 */
const NUMBER: i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
