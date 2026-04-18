//Libraries
use std::fs;
use std::path::Path;

const FILE_NAME: &str = "data.txt";
pub fn init_file(){
    let file_path = Path::new(FILE_NAME);
    if !file_path.exists() {
        let _ = fs::write(FILE_NAME, "0,0");
    }
}

pub fn add_attendance() {
    let file_contents = fs::read_to_string(FILE_NAME).unwrap();
    let data : Vec<_> = file_contents.split(',').collect();
    let data: Vec<i32> = data
        .into_iter()
        .map(|a| a.parse::<i32>().unwrap())
        .collect();
    if data[0] != crate::timelogic::date() {
        let a = format!("{},{}", crate::timelogic::date(), data[1] + 1);
        let _ = fs::write(FILE_NAME, a);
    }
}
