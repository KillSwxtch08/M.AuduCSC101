fn main() {
	let toshiba: f64 = 450_000.0;
	let mac: f64 = 1_500_000.0;
	let hp: f64 = 750_000.0;
	let dell: f64 = 2_850_000.0;
	let acer: f64 = 250_000.0;

	let sum = ( 2.0 * toshiba ) + mac + ( 3.0 * hp ) + ( 3.0 * dell ) + acer;
	println!("Sum is {}", sum);

	let average = sum / 10.0;
	println!("Average is {}", average);

}

