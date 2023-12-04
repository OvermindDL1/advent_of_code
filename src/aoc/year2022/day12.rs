#![allow(clippy::similar_names)]

use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;
use petgraph::algo::astar;
use petgraph::graph::DiGraph;

#[derive(Debug, Parser)]
pub struct Day12 {
	/// The input file of "heightmap"
	#[clap(default_value_t = DataFrom::internal(2022, 12))]
	pub input: DataFrom,
}

fn idx(x: usize, y: usize, width: usize) -> usize {
	y * width + x
}

// fn xy(idx: usize, width: usize) -> (usize, usize) {
// 	(idx % width, idx / width)
// }

impl Day12 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		let input = self.input.as_cow_str()?;
		let input = input.as_ref();

		let width = input.lines().next().context("input is empty")?.trim().len();
		let height = input
			.trim()
			.as_bytes()
			.iter()
			.copied()
			.filter(|&c| c == b'\n')
			.count() + 1;
		// dbg!((width, height));
		let mut gmap: DiGraph<u8, bool> =
			DiGraph::with_capacity(width * height, width * height * 4);
		let mut start = None;
		let mut end = None;

		// Nodes
		let mmap: Vec<_> = input
			.as_bytes()
			.iter()
			.copied()
			.filter(|&c| c.is_ascii_lowercase() || c == b'S' || c == b'E')
			.map(|c| {
				Ok(match c {
					b'S' => {
						#[allow(clippy::eq_op)]
						let weight = b'a' - b'a';
						let node = gmap.add_node(weight);
						start = Some(node);
						(node, weight)
					}
					b'E' => {
						let weight = b'z' - b'a';
						let node = gmap.add_node(weight);
						end = Some(node);
						(node, weight)
					}
					b'a'..=b'z' => (gmap.add_node(c - b'a'), c - b'a'),
					unhandled => bail!("unhandled char: {unhandled}"),
				})
			})
			.collect::<anyhow::Result<_>>()?;

		// Must have found start and end
		let start = start.context("start did not exist in the input")?;
		let end = end.context("end did not exist in the input")?;

		// Edges
		if width * height != mmap.len() {
			bail!(
				"width * height != mmap.len() : {width} * {height} != {}",
				mmap.len()
			);
		}
		for x in 0..width {
			for y in 0..height {
				let this = mmap[idx(x, y, width)];
				if x > 0 {
					let next = mmap[idx(x - 1, y, width)];
					if this.1 + 1 >= next.1 {
						gmap.add_edge(this.0, next.0, true);
						gmap.add_edge(next.0, this.0, false);
					}
				}
				if y > 0 {
					let next = mmap[idx(x, y - 1, width)];
					if this.1 + 1 >= next.1 {
						gmap.add_edge(this.0, next.0, true);
						gmap.add_edge(next.0, this.0, false);
					}
				}
				if x < width - 1 {
					let next = mmap[idx(x + 1, y, width)];
					if this.1 + 1 >= next.1 {
						gmap.add_edge(this.0, next.0, true);
						gmap.add_edge(next.0, this.0, false);
					}
				}
				if y < height - 1 {
					let next = mmap[idx(x, y + 1, width)];
					if this.1 + 1 >= next.1 {
						gmap.add_edge(this.0, next.0, true);
						gmap.add_edge(next.0, this.0, false);
					}
				}
			}
		}

		// dbg!(&gmap);
		// dbg!(&mmap);

		let (total_cost1, _path1) = astar(
			&gmap,
			start,
			|finish| finish == end,
			|e| if *e.weight() { 1 } else { 1000 },
			|n| ((b'z' - b'a') - gmap[n]) as u64,
		)
		.context("no path found")?;
		// dbg!(total_cost1, _path1);

		let (total_cost2, _path2) = astar(
			&gmap,
			end,
			|finish| gmap[finish] == 0,
			|e| if *e.weight() { 1000 } else { 1 },
			|n| gmap[n] as u64,
		)
		.context("no back path found")?;
		// dbg!(total_cost2, _path2);

		Ok((total_cost1, total_cost2))
	}
}
