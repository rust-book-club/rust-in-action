fn main() {

    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 3____0____i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {e}");
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}