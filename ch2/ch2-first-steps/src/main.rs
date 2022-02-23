// this example is to show how to 
// define variables with type annotations
// define & invoke functions
fn main() { // main takes no argument and returns no value
    let a = 10; // variables are immutable by default
    let b: i32 = 20;
    let c = 30i32; // numbers can include type annotation
    let d = 30_i32; // numbers can include underscore for readability
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);
}

fn add(x: i32, y: i32) -> i32 { // funcs must declare return type; Q: what about the main fn?
    x + y // funcs return the last expression's result
}
