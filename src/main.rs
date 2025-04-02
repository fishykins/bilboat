use bilboat::*;

fn main() {
    // Main function for testing the embedded message with correct and incorrect keys
    let original_wav = "input.wav";
    let stego_wav = "stego.wav";
    let secret_message = "Hello, Rust!";
    let key = "super_secret_passphrase"; // Now a string-based key

    embed_message(original_wav, stego_wav, secret_message, key, None);
    println!("Message embedded successfully!");

    let extracted = extract_message(stego_wav, key, None);
    println!("Extracted Message: {}", extracted);

    let extracted_wrong = extract_message(stego_wav, "wrongkey", None);
    println!("Extracted wrong key Message: '{}'", extracted_wrong);
}