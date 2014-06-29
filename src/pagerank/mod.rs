//! Pagerank -- Pagerank Algorithm in Rust

#![crate_id = "pagerank#0.0.1"]
#![crate_type="lib"]
#![allow(unused_must_use)]
#![deny(missing_doc)]
extern crate matrixrs;
use matrixrs::Matrix;
use matrixrs::zeros;
use matrixrs::ToMatrix;

pub fn pagerank(adjm: &Matrix<f64>, rank: &Matrix<f64>, max_iter: uint, q: f64, eps: f64) -> Matrix<f64> {
	//! adjm is the adjacent matrix, and rank is the initial rank matrix,
	//! and max_iter is the maximum iterater time, and q is conventional 0.85,
	//! and eps is the minimum difference bitween two iterations according to L1 norm.
	let adjm = adjm.clone();
	let mut rank = rank.clone();

	let tmp = (adjm * [1f64, 1., 1., 1.].iter().map(|x| x.clone()).to_matrix(1,4).transpose()).map(|v| 1. / v);

	let mut z: Matrix<f64> = zeros(adjm.row, adjm.col);
	for (i,v) in tmp.iter().enumerate() {
		z.set(i+1, i+1 ,v)
	}

	let mt = !(z * adjm);

	let mt = Matrix::from_fn(mt.row, mt.col, |i,j| mt.at(i,j) * q);

	let e = Matrix::from_fn(mt.row, mt.col, |_,_| (1f64-q) / mt.col as f64);
	for _ in range(0,max_iter) {
		let new_rank = (mt + e)* rank;
		if (rank - new_rank).iter().fold(0f64,|acc,b| acc + b.abs()) < eps {
			return new_rank
		} else {
			rank = new_rank
		}
	}
	rank
}


#[test]
fn test_1() {
	let vec: &[f64] = &[
		0., 1., 0., 1.,
		1., 0., 1., 0.,
		0., 1., 0., 0.,
		0., 0., 1., 0.,
		];
	let adjm = vec.iter().map(|x| x.clone()).to_matrix(4,4);

	let mut rank = [0.2f64,0.4,0.2,0.2].iter().map(|x| x.clone()).to_matrix(1,4).transpose();

	rank = pagerank(&adjm,&rank, 1000, 0.85, 0.001); 
	println!("{},{}",rank,rank.iter().fold(0f64,|acc,b| acc+b));  	
	
}
