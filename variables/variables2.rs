/*
if instead of
let x: i32 = 10;

i write

let x = 32

it still passes the tests. but i added explicit types to x because the hint from rustlings asked to explore type annotations, so i did x:i32

on i32: https://doc.rust-lang.org/std/primitive.i32.html
*/

fn main() {
    let x: i32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
