fn main() {
    let names = [
    "Alice Johnson",
    "Bola Adeyemi",
    "Chinedu Okafor",
    "Fatima Yusuf",
    ];

    let years = [5, 12, 8, 15];

    let mut max_years = years[0];
    let mut max_index = 0;

    for i in 0..years.len() {
        if years[i] > max_years {
            max_years = years[i];
            max_index = i;
        }
    }

    println!("Most experienced developer: ");
    println!("Name: {}", names[max_index]);
    println!("Years of Experience: {}", max_years);
}
