use std::thread;

// this example is to show that Rust programs are free from
// Data races
fn main() {
    let mut data = 10;

    // || {} is denoted for a closure
    thread::spawn(|| { data = 100; }); // will throw an error in compile time: closure may outlive the current function
    thread::spawn(|| { data = 1000; }); // will throw an error in compile time: closure may outlive the current function

    println!("{}", data); // will throw an error in compile time: closure may outlive the current function
    // in this case, my Rust extension actually is able to show the errors even before running cargo run
}
