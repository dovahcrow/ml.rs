#![feature(globs)]
extern crate gnuplot;
use std::iter;
use gnuplot::*;

fn main() {
	let x = range(1.0f32, 8.0);
	let y1: Vec<f32> = x.map(|v| { let z = v - 4.0; z * z - 5.0}).collect();
	let y1 = y1.iter();
	let y2: Vec<f32> = x.map(|v| { let z = v - 4.0; -z * z + 5.0 }).collect();
	let y2 = y2.iter();
	let y3: Vec<f32> = x.map(|v| { v - 4.0 }).collect();
	let y3 = y3.iter();

	let mut fg = Figure::new();

	fg.axes2d()
	.set_size(1.0, 1.0)
	.set_title("Example Plot", [TextColor("blue")])
	.set_x_ticks(Some((Fix(1.5), 1)), [Mirror(false)], [])
	.set_y_ticks(Some((Fix(1.5), 1)), [Mirror(false)], [])
	.set_legend(Graph(1.0), Graph(0.5), [Placement(AlignLeft, AlignCenter)], [TextAlign(AlignRight)])
	.set_border(true, [Left, Bottom, Top, Right], [LineWidth(2.0)])
	.set_x_label("Abscissa", [])
	.set_y_label("Ordinate", [])
	.arrow(Axis(5.7912), Axis(2.7912), Axis(5.7912), Axis(1.7912), [ArrowType(Closed), ArrowSize(0.1), LineWidth(2.0), Color("black")])
	.label("Here", Axis(5.7912), Axis(3.1), [TextAlign(AlignCenter)])
	.fill_between(x, y1.map(|&y| y * 0.85 - 1.0), y1.map(|&y| y * 1.15 + 1.0), [Color("#aaaaff")])
	.lines(x, y1, [Caption("(x - 4)^2 - 5"), LineWidth(1.5), Color("black")])
	.y_error_lines(x, y2, iter::Repeat::new(1.0f32), [Caption("(x - 4)^2 + 5"), LineWidth(1.5), Color("red")])
	.lines_points(x, y3, [Caption("x - 4"), PointSymbol('t'), LineWidth(1.5), LineStyle(Dash), Color("#11ff11")]);
	fg.show();

}