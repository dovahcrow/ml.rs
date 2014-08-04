#![feature(globs)]
extern crate gnuplot;
use gnuplot::figure::Figure;
use gnuplot::AxesCommon;
use gnuplot::options::*;
// use gnuplot::coordinates::*;

fn main() {
	let x: Vec<f64> = range(0i,100).map(|x| x as f64 / 20f64).collect();
	let y: Vec<f64> = x.iter().map(|&x| (x as f64).sin()).collect();

	let mut figure = Figure::new();
	figure.axes2d()
	.set_size(1.0, 1.0)
	.set_title("sin(x)", [])
	.set_border(true, [Left, Bottom, Top, Right], [LineWidth(2.0)])
	.points(x.iter(), y.iter(), [Caption("sin(x)"), LineWidth(1.5), Color("black")]);
	figure.show();
}