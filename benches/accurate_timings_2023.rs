use advent_of_code::aoc::helpers::DataFrom;
use advent_of_code::aoc::year2023::*;
use advent_of_code::*;
use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};

const AOCAPP: AocApp = AocApp {
	verbose: 0,
	hide_scores: true,
	command: AocAppCommand::TUI,
};

fn gen_input(year: u16, day: u8) -> DataFrom {
	let input = DataFrom::internal(year, day);
	input
		.preload()
		.expect("preload from internal should never fail");
	input
}

macro_rules! bench_day {
	($benchname:ident, $year:literal, $day:literal, $daystruct:tt) => {
		#[library_benchmark]
		#[bench::short(gen_input($year, $day))]
		fn $benchname(input: DataFrom) {
			let day = $daystruct { input };
			let res = day.run(&AOCAPP);
			let _ = black_box(res);
		}
	};
}

bench_day!(y2023d01, 2023, 1, Day1);
bench_day!(y2023d02, 2023, 2, Day2);
bench_day!(y2023d03, 2023, 3, Day3);
bench_day!(y2023d04, 2023, 4, Day4);
bench_day!(y2023d05, 2023, 5, Day5);

library_benchmark_group!(
	name = bench_year2023;
	benchmarks = y2023d01, y2023d02, y2023d03, y2023d04, y2023d05
);

main!(library_benchmark_groups = bench_year2023);
