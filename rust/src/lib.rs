
#[no_mangle]
pub fn will_return_string() -> String {
    String::from("Hello from Rust")
}

#[no_mangle]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
