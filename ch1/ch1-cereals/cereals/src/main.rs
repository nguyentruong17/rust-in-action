#[derive(Debug)] // allows the println! macro to print this enum, but is it like the toString in java?
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat
}

// this example is to show that Rust programs are free from
// Dangling pointer
fn main() {
    let mut grains: Vec<Cereal> = vec![]; // init an empty vector of cereals, but what does the vec! do?
    grains.push(Cereal::Barley);
    drop(grains);
    println!("{:?}", grains); // will throw an error in compile time: borrow of moved value `grains`
    // in this case, my Rust extension actually is able to show the error even before running cargo run
}
