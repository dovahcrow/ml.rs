use super::{Distribution,DistributionIterator};
use std::rand::{Rng,task_rng};
use std::iter::range_inclusive;
use std::num::{one,Bounded,ToPrimitive};
use std::rand::TaskRng;


pub struct Poisson<T> {
	task_rng: TaskRng,
	lambda: uint,
}

impl<T:Num+Bounded+ToPrimitive+PartialOrd+Clone> Poisson<T> {
	pub fn new(lambda: uint) -> Poisson<T> {
		Poisson {
			task_rng: task_rng(),
			lambda: lambda
		}
	}
	pub fn iter(&mut self) -> DistributionIterator<T> {
		DistributionIterator {
			distro: self
		}
	}
}

impl<T:Num+Bounded+ToPrimitive+PartialOrd+Clone> Distribution<T> for Poisson<T> {
	fn emit(&mut self) -> T {
		let x: f64 = self.task_rng.gen();

		let k = range_inclusive(one(), Bounded::max_value()).find(
			|k: &T| {
				let fractorial = range_inclusive(one(), k.clone()).fold(one(), |acc: T, next_k| acc * next_k);
				x < (self.lambda as f64).powi(k.to_i32().unwrap()) / (fractorial.to_f64().unwrap()) * (-(self.lambda as int) as f64).exp() 
			});
		k.unwrap()
	}
}

#[test]
fn testPoisson() {
	let poisson: Poisson<uint> = Poisson::new(3);
	fail!("{}", poisson.emit());
}