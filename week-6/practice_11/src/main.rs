fn main() {
    let a:i32 = 2;
    let b:i32 = 3;

    let mut result:f32;

    result = (a & b) as f32;
    println!("(a & b) => {}", result);

    result = (a | b) as f32;
    println!("(a | b) => {}", result);

    result = (a ^ b) as f32;
    println!("(a ^ b) => {}", result);

    result = (!b) as f32;
    println!("(!b) => {}", result);

    result = (a << b)  as f32;
    println!("(a << b) => {}", result);

    result = (a >> b) as f32;
    println!("(a >> b) => {}", result);
}
