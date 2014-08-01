#![experimental]
extern crate matrixrs;
use self::matrixrs::Matrix;
use std::num::Zero;

pub struct C45<T> {
	pub data: Matrix<T>,
}

impl<T:Zero> C45<T> {
	pub fn new(row: uint, col: uint) -> C45<T> {

		C45 {data: Matrix::from_fn(row, col, |_, _| {Zero::zero()})}

	}
}