use std::thread::sleep;
use std::time::Duration;

#[no_mangle]
pub fn say_hello() {
    println!("Say Hello World!");
}

#[no_mangle]
pub fn say_hello_2() -> String {
    "Return Hello World!".to_string()
}

#[no_mangle]
pub fn echo(message: String) -> String {
    "Echo ".to_string() + &message
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub fn square(a: i32) -> i32 {
    a * a
}

#[no_mangle]
pub fn loop_hello() {
    for n in 0..5 {
        println!("{} Hello World!", n);
    }
}

#[no_mangle]
pub fn delay_loop_hello() {
    for n in 0..5 {
        println!("{} Hello World!", n);
        sleep(Duration::from_secs(1));
    }
}
