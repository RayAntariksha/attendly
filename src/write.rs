use crate::timelogic;
use std::fs::OpenOptions;
use std::fs;
use std::io::{self, Read, Write};

pub fn write_to_file(file_name: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name);

    let mut content = String::new();
    file?.read_to_string(&mut content)?;
    let content: &str = content.trim();

    let parts: Vec<&str> = content.split(',').collect();
    let num: i32 = parts[0]
        .parse()
        .expect("failed to convert date to a integer");

    if attendance(content) == true {
        let entry = format!("{},{}", num+1, timelogic::date());
        fs::write(file_name, entry)?;
    }

    Ok(())
}
fn attendance(content: &str) -> bool {
    let parts: Vec<&str> = content.split(',').collect();
    let date: i32 = parts[1]
        .parse()
        .expect("failed to convert date to a integer");
    if date == timelogic::date() {
        return false;
    } else {
        return true;
    }
}
