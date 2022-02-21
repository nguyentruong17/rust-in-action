fn greet_world() {
    println!("Hello, world!");
    let southern_german = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_german, japan];
    for region in regions.iter() {
        println!("{}", &region); // with the ampersand, it is read-only
    }
}
fn main() {
    greet_world();
}
