// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::decode;
use image::load_from_memory;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str){
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());
}