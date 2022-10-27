 fn main() {
	let p:f64 =520_000_000.00 ;
	let r:f64 = 10.00 ;
	let n:f64 = 5.00 ;
	// where P= Principal, R=rate, T= time (years)

	let x = 1.0 + (r/100.00); // Sub the equation ( 1.0 + (r/100.00)) for x from the formula of C.I
	let y = x.powf(n) ;
	let amount = p * y;
	let ci = amount - p;
	println!("The Compound Interest for 5 years is {}",ci );
} 