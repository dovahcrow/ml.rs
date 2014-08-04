use std::iter::Iterator;
pub mod poisson;

pub trait Distribution<T> {
	fn emit(&mut self) -> T;
}

pub struct DistributionIterator<'a, T> {
	distro: &'a mut Distribution<T>
}

impl<'a,T> Iterator<T> for DistributionIterator<'a,T> {
	fn next(&mut self) -> Option<T> {
		Some(self.distro.emit())
	}
}