fn main() {
	let t:f64 = 2.0 * 450000.00;
	let d:f64 = 3.0 * 2850000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 3.0 * 750000.00;
	let a:f64 = 250000.00;

	// Average
	let s = t + d + m + h + a;
	let n = 2.0 + 1.0 + 3.0 + 1.0;
	let v = s / n;

	println!("Sum is {}", s);
	println!("Average is{}",v);
}