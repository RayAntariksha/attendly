//Libraries
//Modules
mod timelogic;
mod write;

// OPENING_TIME and CLOSING_TIME are in 24-hour format
// (only the hour to be used)
const OPENING_TIME: i32 = 11;
const CLOSING_TIME: i32 = 17;


/// Simple program to greet a person
fn main() {
    write::init_file();
    logic();
}

fn logic() {
    let is_schooltime = timelogic::school_time(OPENING_TIME, CLOSING_TIME);
    if is_schooltime {
        write::add_attendance();
    }else {
        println!("It is not school time");
    }
}
