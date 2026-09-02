pub fn solve() {
	for n in 1000..9999 {
		if !has_different_valid_digits(n) {
			continue;
		}
		let sx = n / 10;
		let dx = n % 10;
		let result = sx * dx;
		if result < 1000 {
			continue;
		}
		let t = n * 10000 + result;
		if has_different_valid_digits(t) {
			println!("{sx} x {dx} = {result}")
		}
	}
}

fn has_different_valid_digits(mut n: u32) -> bool {
	let mut seen = 0u32;
	while n > 0 {
		let digit = (n % 10) as usize;
		if digit == 0 || digit == 9 {
			return false;
		}
		let mask = 1 << digit;
		if seen & mask != 0 {
			return false;
		}
		seen |= mask;
		n /= 10;
	}
	true
}