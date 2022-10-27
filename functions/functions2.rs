/*
just added a type to pair with the parameter name (num)
"fn call_me(num:i32) {"
*/

fn main() {
    call_me(3);
}

fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
