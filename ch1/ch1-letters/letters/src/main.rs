// this example is to show that Rust programs are free from
// Iterator invalidation
fn main() {
    let mut letters = vec!["A", "B", "C"];

    for letter in letters {
        println!("{}", letter);
        // why cloning a str?
        letters.push(letter.clone()); // will throw an error in compile time: borrow of moved value `letters`
    }
    // in this case, my Rust extension actually is able to show the error even before running cargo run
}
