
#[no_mangle]
pub fn will_return_nothing() -> String {
    String::from("Hello from Rust")
}

#[no_mangle]
pub fn will_panic(arr: &mut [u8]) {
    arr[0] = 10;
}

#[no_mangle]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
