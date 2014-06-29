extern crate debug;


// struct Matrix<T> {
// 	data : Vec<Vec<T>>
// }

fn main() {
	let a = vec!(1,2,3);
	let b = a.as_slice();


	println!("{:?}",b[1]);
}