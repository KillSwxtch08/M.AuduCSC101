use std::io;

fn main() {
    println!("Enter your value for a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your value for b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid string");

    println!("Enter your value for c: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid string");

    let discriminant = b * b - (4.0 * a * c);
    println!("The discriminant is {}", discriminant );

    if discriminant > 0.0 {
        println!("The equations has two real roots");
}
        else if discriminant == 0.0 {
            println!("The equation has one real root");
}
            else {
                println!("The equation has no real root");
            }
            } 
                
            
        
    

