pub trait Distribution<T> {
	pub fn emit(&self) -> T 
}