fn main() {
    let a:f32 = 20.0;
    let b:f32 = 30.0;

    if (a > 10.0) && (b > 10.0) {
        println!("true");
    }

    let c:f32 = 0.0;
    let d:f32 = 30.0;

    if (c>10.0) || (d>10.0){
        println!("true");
    }
    let is_elder = false;

    if !is_elder {
        println!("Not Elder");
    }
}