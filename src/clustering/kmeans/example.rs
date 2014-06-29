extern crate kmeans;
extern crate matrixrs;
use matrixrs::ToMatrix;

fn main() {
	let a: &[int] = [
	1,2,3,4,
	5,6,7,8,
	9,20,11,12,
	13,14,15,16
	];
	let center = kmeans::KMeans::gen_center(a.iter().map(|x| x.clone()).to_matrix(4,4));
	println!("{}",center);
}