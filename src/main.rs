use bilboat::*;

fn main() {
    // Main function for testing the embedded message with correct and incorrect keys
    let secret_message = "Hello, Rust!";
    let key = "super_secret_passphrase";

    let input_wav = WavBuffer::from_file("input.wav").unwrap();
    //let input_wav = WavBuffer::sin(1);

    let output_wav = embed_message(&input_wav, secret_message, key, Encryption::default())
        .expect("Failed to embed wav");
    println!("Message embedded successfully!");

    let extracted = extract_message(&output_wav, key, Encryption::default());
    println!("Extracted Message: {}", extracted);

    let extracted_wrong = extract_message(&output_wav, "wrongkey", Encryption::default());
    println!("Extracted wrong key Message: '{}'", extracted_wrong);

    output_wav
        .write_to_file("output.wav")
        .expect("Failed to write file");
}
