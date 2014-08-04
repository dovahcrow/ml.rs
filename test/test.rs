#![feature(globs)]
extern crate gnuplot;
extern crate ml;
use gnuplot::figure::Figure;
use gnuplot::AxesCommon;
use gnuplot::options::*;
use std::rand::task_rng;
use std::rand::Rng;
// use gnuplot::coordinates::*;

fn main() {
	use ml::statistics::distributions::poisson::Poisson;
	use ml::statistics::distributions::Distribution;

	let mut p: Poisson<uint> = Poisson::new(3);

	println!("{}", p.emit());

	let x: Vec<f64> = range(0i,100).map(|x| x as f64).collect();
	let y: Vec<f64> = x.iter().map(|_| p.iter().next().unwrap().to_f64().unwrap()).collect();

	let mut figure = Figure::new();
	figure.axes2d()
	.set_size(1.0, 1.0)
	.set_title("sin(x)", [])
	.set_border(true, [Left, Bottom, Top, Right], [LineWidth(2.0)])
	.points(x.iter(), y.iter(), [Caption("sin(x)"), LineWidth(1.5), Color("black")]);
	figure.show();
}
