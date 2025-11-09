use std::io;

fn main() {
    println!("-------PAU  MENU-------");
    println!("Code\t\tMenu\t\t\t\t\tPrice(₦)");
    println!("P\t\tPoundo Yam/Edinkaiko Soup\t\t3,200");
    println!("F\t\tFried Rice & Chicken\t\t\t3,000");
    println!("A\t\tAmala & Ewedu Soup\t\t\t2,500");
    println!("E\t\tEba & Egusi Soup\t\t\t2,000");
    println!("W\t\tWhite Rice & Stew\t\t\t2,500");

    let mut food = String::new();
    println!("Enter food code (P/F/A/E/W): ");
    io::stdin().read_line(&mut food).expect("Error");
    let food = food.trim().to_uppercase();

    let mut quantity = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity).expect("Error");
    let quantity: u32 = quantity.trim().parse().expect("Error");

    let mut price = 0;

    if food == "P" {
        price = 3200;
    }
    else if food == "F"{
        price = 3000;
    }
    else if food == "A"{
        price = 2500;
    }
    else if food == "E"{
        price = 2000;
    }
    else if food == "W"{
        price = 2500;
    }
    else{

    println!("Invalid food code");
    return;}

    let mut total = price * quantity;

    if total > 10_000 {
        let discount = (total as f64) * 0.05;
        total = (total as f64 - discount) as u32;
        println!("\nA 5% discount has been applied");
    }

    println!("\n------RECEIPT------");
    println!("Food Code: {}", food);
    println!("Quantity: {}", quantity);
    println!("Total: ₦{}", total);
    println!("----------------------");
    }
