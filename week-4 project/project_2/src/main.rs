use std::io;

fn main() {
    println!("Whats your experience: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience = input1.trim();

    // I put the age question in a nested if function so as to skip and separate this from people who are inexperienced, so it wont need to ask for age
    if experience == "experienced" {
     println!("how old are you? ");
 
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age: f32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40.0 {
        println!("The employee is {} and is {} years old.",experience,age);
        println!("Therefore, the employee's incentive is N1,560,000");
    } 

    else if age >= 30.0 && age < 40.0 {
        println!("The employee is {} and is {} years old.",experience,age);
        println!("Therefore, the employee's incentive is N1,480,000");
    }

    else if age < 28.0 {
        println!("The employee is {} and is {} years old.",experience,age);
        println!("Therefore, the employee's incentive is N1,300,000 per month");
    }

}

        else if experience == "inexperienced" {
        println!("The employee is {}",experience);
        println!("The employee's incentive is N100,000");

    }
}
