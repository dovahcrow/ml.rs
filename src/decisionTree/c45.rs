#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case_functions)]
#![allow(uppercase_variables)] 
extern crate debug;

mod c45 {
	use super::matrix::Matrix;
	use std::num::Zero;


	pub struct C45<T> {
		pub data: Matrix<T>,
	}

	impl<T:Zero> C45<T> {
		pub fn new(row: uint, col: uint) -> C45<T> {

			C45 {data: Matrix::from_fn(row, col, |_, _| {Zero::zero()})}

		}
	}

}



fn main() {
	// use c45::C45;
	// let a: C45<int> = C45::new(2,2);
	let m = matrix::Matrix::from_T(4,4,0);
	println!("{}",m);
}	