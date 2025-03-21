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

// Import necessary libraries
use base64::{decode, encode}; // For encoding and decoding Base64 strings
use image::ImageOutputFormat::Png; // For specifying PNG output format
use image::load_from_memory; // For loading an image from bytes
use wasm_bindgen::prelude::*; // For enabling WebAssembly (WASM) binding
use web_sys::console::log_1 as log; // For logging messages to the browser console

// Define a function that will be exposed to JavaScript using WebAssembly
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    // Log message to console: function has been called
    log(&"Grayscale called".into());

    // Decode the Base64-encoded image string into a byte vector
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    // Load the image from the byte vector
    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    // Convert the image to grayscale
    image = image.grayscale();
    log(&"Grayscale effect applied".into());

    // Create an empty buffer to store the processed image
    let mut buffer = vec![];
    
    // Write the processed grayscale image to the buffer in PNG format
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    // Encode the buffer (image bytes) back into a Base64 string
    let encoded_image = encode(&buffer);
    
    // Create a data URL (base64-encoded image) that can be used in HTML (e.g., <img src="data:image/png;base64,...">)
    let data_url = format!("data:image/png;base64,{}", encoded_image);

    // Return the processed image as a Base64 data URL
    data_url
}

