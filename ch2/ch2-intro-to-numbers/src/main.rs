fn main() {
    let twenty = 20; // rust will infer type
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    // here, rust infers twenty as i32 type
    // if twenty_two is declared as i64, then the operation below
    // will throw an error in compile time: mismatched types - expected i64, found i32 for twenty_two
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // let forty: i64 = 40;
    // here, because rust already inferred twenty as i32, the operation below
    // will throw an error in compile time: mismatched types - expected i64, found i32 for forty
    // addition = twenty + forty;
    
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2)); // numbers have methods!

    // here, rust infers forty_fours as vec<f32>
    // in rust, elements within an array must be of the same type
    let forty_fours = [
        44.0,
        44f32,
        44.0_f32
    ];

    println!("{}", forty_fours[0]); // will print 44 instead of 44.0, why?
}
