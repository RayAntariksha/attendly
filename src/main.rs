mod write;
mod timelogic;

const FILE_NAME: &str = "data.txt";
const OPENING_TIME: i32 = 11;
const CLOSING_TIME: i32 = 17;
fn main() {
    logic();
    println!("Program Finished!");
}

fn logic() {
    let command = timelogic::school_time(OPENING_TIME, CLOSING_TIME);
    if command == true{
        #[allow(unused_must_use)]
     write::write_to_file(FILE_NAME);
    }
    
}

