use std::fs;
use std::io::{self, Read, Write};

const FILE_NAME: &str = "data.txt";
fn main() {
    write_to_file().unwrap();
}

fn write_to_file() -> io::Result<()> {
    let mut stored_number: i32 = 0;

    // Try to read the number from the file
    if let Ok(mut file) = fs::File::open("data.txt") {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        if let Ok(num) = contents.trim().parse::<i32>() {
            stored_number = num;
            println!("Loaded number: {}", stored_number);
        } else {
            eprintln!("Error: Could not parse number from file.");
        }
    } else {
        println!("File not found. Starting with default number.");
    }

    // Modify the number (for demonstration)
    stored_number += 1;
    println!("New number: {}", stored_number);

    // Save the updated number back to the file
    let mut file = fs::File::create(FILE_NAME)?;
    file.write_all(stored_number.to_string().as_bytes())?;
    println!("Number saved to file.");

    Ok(())
}
