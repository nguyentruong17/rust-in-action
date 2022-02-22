// this example is to show that Rust programs are free from
// Buffer overflow

fn main() {
    let fruits = vec!["Apple", "Banana", "Cherry"];

    let buffer_overflow = fruits[4];
    assert_eq!(buffer_overflow, "Durian"); // will throw an error in compile time: index out of bounds
}
