pub trait Distribution<T> {
	fn emit(&self) -> T;
}