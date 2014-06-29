//! Kmeans -- Kmeans Algorithm in Rust

#![crate_id = "kmeans#0.0.1"]
#![crate_type="lib"]
#![allow(unused_must_use)]
#![allow(missing_doc)]
extern crate matrixrs;
use matrixrs::{Matrix,ToMatrix};
use std::num::{zero,Num};

pub struct KMeans<T> {
	data: Matrix<T>,
	dimension: (uint,uint)
}

impl<T:Clone+Num> KMeans<T> {
	pub fn new(data: Matrix<T>) -> KMeans<T> {
		KMeans {
			dimension: data.size(),
			data: data
		}
	}

	pub fn gen_center(points: Matrix<T>) -> Matrix<T> {
		let mut rev_points = !points;

		rev_points.rows().map(
			|row| -> T {
				row.iter().fold(zero(),|acc: T, b| acc + *b)
		}).to_matrix(1, points.col) 
	}

	pub fn update_centers(&self,assignments) {

	
	}
    
        // new_means = collections.defaultdict(list)

        // centers = []
        // for assignment, point in zip(assignments, self.__matrix):
        //     new_means[assignment].append(point)

        // for points in new_means.values():
        //     centers.append(self.__point_avg(points))

        // return centers
}
