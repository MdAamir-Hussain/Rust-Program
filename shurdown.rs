use std::process;

fn main() {
    let num = 1;
    if num == 1 {
        println!("Shutting down...");
        process::exit(0);
    } else {
        println!("Try again!");
    }
}
