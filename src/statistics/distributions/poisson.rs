struct Poisson<T> {
	lambda: int,
}
impl Poisson<uint> {
	fn new(lambda: int) -> {
		Poisson {
			lambda: lambda
		}
	}
}
impl Distribution for Poisson<uint> {
	fn emit(&self) -> uint {
		0
	}
}

#[test]
fn testPoisson() {
	let poisson = Poisson::new(3);
	fail!("{}", poisson.emit());
}