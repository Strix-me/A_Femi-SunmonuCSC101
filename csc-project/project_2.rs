 fn main() {
	let t:f64 = 450_000.00 * 2.0;
	let m:f64 = 1_500_000.00 * 1.0;
	let hp:f64 = 750000.00 * 3.0;
	let d:f64 = 2_850_000.00 * 3.0;
	let a:f64 = 250_000.00 * 1.0;
	// where t= Toshiba, m= Mac, Hp= Hp, d= Dell, a= Acer

	let sum = t + m + hp + d + a;
	let average = sum/5.0; //where 5 is the total number of Computer Brands.

	println!("The Sum of the amount is {}, The Average is {} ",sum, average, );
	
}