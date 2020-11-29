use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn will_return_string() -> String {
//     String::from("Hello from Rust")
// }

#[wasm_bindgen]
pub fn will_modify(arr: &mut [u8]) {
    arr[0] = 10;
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
