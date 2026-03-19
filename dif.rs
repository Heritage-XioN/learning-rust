fn main() {
    let re: &str = longest("hello world", "yo what");

    println!("{}", re);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let mut z: &str = "";
    if x.len() > y.len() {
        z = x;
    } else {
        z = y;
    }

    z
}
