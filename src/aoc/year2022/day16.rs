/// Something here is not right in part 2, even though it solves my input...
use crate::aoc::helpers::*;
use crate::AocApp;
use ahash::{HashMap, HashMapExt};
use anyhow::{bail, Context};
use clap::Parser;
use nalgebra::DMatrix;
use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug, Parser)]
pub struct Day16 {
	/// The input file of "valve information"
	#[clap(default_value_t = DataFrom::internal(2022, 16))]
	pub input: DataFrom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ID([u8; 2]);
impl Display for ID {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
	id: ID,
	flow_rate: u8,
	nexts: [Option<ID>; 5],
}

impl Day16 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let input = self.input.as_cow_str()?;
		let input = input.as_ref();

		let mut nodes = HashMap::new();
		let mut wanted_nodes = Vec::new();
		let mut graph = DiGraph::new();

		for line in input.lines() {
			let ("Valve ", line) = line.split_at(6) else {
				bail!("invalid valve line: {}", line);
			};
			let (id, line) = line.split_at(2);
			let id = ID(id.as_bytes().try_into().context("invalid ID format")?);
			let (" has flow rate=", line) = line.split_at(15) else {
				bail!("invalid flow rate line: {}", line);
			};
			let (flow_rate, line) = line.split_once(';').context("invalid flow rate format")?;
			let flow_rate = flow_rate.parse().context("flow rate is not a number")?;
			let mut valve = Valve {
				id,
				flow_rate,
				nexts: [None; 5],
			};
			let at = line
				.bytes()
				.enumerate()
				.filter(|(_, b)| *b == b' ')
				.nth(4)
				.context("invalid valve line")?
				.0;
			let (_, line) = line.split_at(at + 1);
			for (i, node) in line.split(", ").enumerate() {
				if i >= 5 {
					bail!("too many tunnels: {}", line);
				}
				let id = ID(node
					.as_bytes()
					.try_into()
					.context("invalid edge ID format")?);
				valve.nexts[i] = Some(id);
			}
			let node = graph.add_node((valve.id, valve.flow_rate));
			if valve.id.0 == *b"AA" {
				wanted_nodes.push((node, valve.flow_rate as usize));
				let len = wanted_nodes.len();
				wanted_nodes.swap(0, len - 1);
			} else if valve.flow_rate > 0 {
				wanted_nodes.push((node, valve.flow_rate as usize));
			}
			nodes.insert(valve.id, (valve, node));
		}

		for (valve, node) in nodes.values() {
			for next in valve.nexts.iter().flatten() {
				graph.add_edge(*node, nodes[next].1, ());
			}
		}

		let mut paths = DMatrix::zeros(wanted_nodes.len(), wanted_nodes.len());
		for (i0, (n0, _)) in wanted_nodes.iter().copied().enumerate() {
			for (i1, (n1, _)) in wanted_nodes.iter().copied().enumerate() {
				if i0 == i1 {
					continue;
				}
				let Some((weight, _path)) =
					petgraph::algo::astar(&graph, n0, |n| n == n1, |_e| 1, |_n| 1u8)
				else {
					continue;
				};
				paths[(i0, i1)] = weight;
			}
		}

		if app.verbose > 1 {
			std::fs::write(
				"2022-16.dot",
				format!("{:?}", Dot::with_config(&graph, &[])),
			)?;
		}
		// dbg!(&graph);
		// for xs in paths.row_iter() {
		// 	for x in &xs {
		// 		print!("{:3} ", x);
		// 	}
		// 	println!();
		// }

		let mut remaining: VecDeque<_> = (1..wanted_nodes.len()).collect();
		let score1 = find_best_score_in_time(&paths, &wanted_nodes, 0, 0, 0, &mut remaining);
		let mut remaining: VecDeque<_> = (1..wanted_nodes.len()).collect();
		let score2 = find_best_score_in_time_dual(
			&paths,
			&wanted_nodes,
			0,
			0,
			(0, 0),
			CurState::Neither,
			&mut remaining,
		)?;
		// for i in 0..wanted_nodes.len() {
		// 	print!("{i:<2} ");
		// }
		// println!();
		// for node in &wanted_nodes {
		// 	let id = graph[node.0].0;
		// 	print!("{id} ");
		// }
		// println!();

		// let counts: Vec<usize> = (1..wanted_nodes.len()).collect();
		// let mut best_score = 0;
		// let mut best_path = Vec::with_capacity(wanted_nodes.len());
		// let mut cache_path = Vec::with_capacity(wanted_nodes.len());
		// for perm in permute::permutations_of(&counts) {
		// 	// {
		// 	// 	let get = |id: &[u8; 2]| {
		// 	// 		let node = nodes[id].1;
		// 	// 		wanted_nodes.iter().position(|&n| n == node).unwrap()
		// 	// 	};
		// 	// 	let perm = [
		// 	// 		// get(b"AA"),
		// 	// 		get(b"DD"),
		// 	// 		get(b"BB"),
		// 	// 		get(b"JJ"),
		// 	// 		get(b"HH"),
		// 	// 		get(b"EE"),
		// 	// 		get(b"CC"),
		// 	// 	];
		// 	// 	let perm = perm.iter();
		// 	let perm = perm.collect::<Vec<_>>();
		// 	// for p in &perm {
		// 	// 	let id = graph[wanted_nodes[**p]].0;
		// 	// 	print!("{}{} ", id[0], id[1]);
		// 	// }
		// 	// print!("-> ");
		// 	let perm = perm.iter().copied();
		// 	cache_path.clear();
		// 	let mut prior = 0;
		// 	let mut rate = 0u64;
		// 	let mut score = 0u64;
		// 	let mut time = 1;
		// 	for i in perm.copied() {
		// 		let t = paths[(prior, i)] as u64 + 1; // +1 for activating `i` valve
		// 							  // println!("Minute {time}");
		// 							  // for tt in 1..=t {
		// 							  // 	println!(
		// 							  // 		"Valves {:?} are open, releasing {rate} pressure, {} released so far",
		// 							  // 		cache_path
		// 							  // 			.iter()
		// 							  // 			.map(|&i| wanted_nodes[i])
		// 							  // 			.map(|i| graph[i])
		// 							  // 			.map(|i: ([char; 2], _)| format!("{}{}", i.0[0], i.0[1]))
		// 							  // 			.collect::<Vec<_>>(),
		// 							  // 		score + (rate * tt)
		// 							  // 	);
		// 							  // }
		// 							  // println!(
		// 							  // 	"Moved to {}{} and opening valve to change the rate to {}\n",
		// 							  // 	graph[wanted_nodes[i]].0[0],
		// 							  // 	graph[wanted_nodes[i]].0[1],
		// 							  // 	rate + graph[wanted_nodes[i]].1 as u64
		// 							  // );
		// 		if time + t > 30 {
		// 			score += rate * (31 - time);
		// 			time = 30;
		// 			break;
		// 		}
		// 		score += rate * t;
		// 		rate += graph[wanted_nodes[i]].1 as u64;
		// 		time += t;
		// 		cache_path.push(i);
		// 		prior = i;
		// 		// if cache_path[0] == 3 && (cache_path.len() == 1 || cache_path[1] == 1) {
		// 		print!("{i}:{t}:{time}:{rate}:{score}  ");
		// 		// }
		// 	}
		// 	if time < 30 {
		// 		score += rate * (31 - time);
		// 		time = 30;
		// 	}
		// 	println!("-> {time} -> {rate} -> {score}");
		// 	if score > best_score {
		// 		best_score = score;
		// 		best_path.clear();
		// 		best_path.extend(cache_path.iter().copied());
		// 	}
		// }
		// dbg!(best_score);
		// let mut prior = 0;
		// print!("AA");
		// for i in best_path.iter().copied() {
		// 	let id = &graph[wanted_nodes[i]].0;
		// 	print!(" {}> {}{}", paths[(prior, i)] + 1, id[0], id[1]);
		// 	prior = i;
		// }
		// println!();

		Ok((score1, score2))
	}
}

fn find_best_score_in_time(
	travel_times: &DMatrix<u8>,
	rates: &Vec<(NodeIndex, usize)>,
	time: u8,
	rate: usize,
	prior: usize,
	remaining: &mut VecDeque<usize>,
) -> usize {
	// println!(
	// 	"{}{cur} {time} {rate} {remaining:?}",
	// 	" ".repeat(20 - remaining.len()),
	// );
	// Default best score is if we don't move, spend remaining time at current node
	let mut best_score = (30 - time as usize) * rate;
	for _ in 0..remaining.len() {
		let cur = remaining.pop_front().unwrap();
		let t = travel_times[(prior, cur)] + 1; // + 1 for activating the valve
		let score = if time + t > 30 {
			rate * (30 - time as usize)
		} else {
			rate * t as usize
				+ find_best_score_in_time(
					travel_times,
					rates,
					time + t,
					rate + rates[cur].1,
					cur,
					remaining,
				)
		};
		best_score = best_score.max(score);
		remaining.push_back(cur);
	}
	// println!("{}->{best_score}", " ".repeat(20 - remaining.len()),);
	best_score
}

enum CurState {
	Neither,
	Left(u8, usize),
	Right(u8, usize),
	Both((u8, usize), (u8, usize)),
}

impl Display for CurState {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			CurState::Neither => write!(f, "Neither"),
			CurState::Left(t0, cur0) => write!(f, "Left({t0},{cur0})"),
			CurState::Right(t1, cur1) => write!(f, "Right({t1},{cur1})"),
			CurState::Both((t0, cur0), (t1, cur1)) => {
				write!(f, "Both(({t0},{cur0}),({t1}, {cur1}))")
			}
		}
	}
}

const MAX_DUAL_TIME: u8 = 26;

#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
fn find_best_score_in_time_dual(
	travel_times: &DMatrix<u8>,
	rates: &Vec<(NodeIndex, usize)>,
	time: u8,
	rate: usize,
	prior: (usize, usize),
	cur: CurState,
	remaining: &mut VecDeque<usize>,
) -> anyhow::Result<usize> {
	if time > MAX_DUAL_TIME {
		bail!("too much time spent {time} > {MAX_DUAL_TIME}");
	}
	let mut best_score = (MAX_DUAL_TIME - time) as usize * rate;
	match cur {
		CurState::Neither => {
			for _ in 0..remaining.len() {
				let cur0 = remaining.pop_front().unwrap();
				let t0 = travel_times[(prior.0, cur0)] + 1; // + 1 for activating the valve
											// Test what happens if right does nothing from here on out
				let score = find_best_score_in_time_dual(
					travel_times,
					rates,
					time,
					rate,
					(prior.0, 0),
					CurState::Both((t0, cur0), (30, 0)),
					remaining,
				)?;
				best_score = best_score.max(score);
				for _ in 0..(remaining.len()) {
					let cur1 = remaining.pop_front().unwrap();
					let t1 = travel_times[(prior.1, cur1)] + 1; // + 1 for activating the valve
					let score = find_best_score_in_time_dual(
						travel_times,
						rates,
						time,
						rate,
						(prior.0, prior.1),
						CurState::Both((t0, cur0), (t1, cur1)),
						remaining,
					)?;
					best_score = best_score.max(score);
					remaining.push_back(cur1);
				}
				remaining.push_back(cur0);
			}
			// Test what happens if left does nothing from here on out
			for _ in 0..(remaining.len()) {
				let cur1 = remaining.pop_front().unwrap();
				let t1 = travel_times[(prior.1, cur1)] + 1; // + 1 for activating the valve
				let score = find_best_score_in_time_dual(
					travel_times,
					rates,
					time,
					rate,
					(0, prior.1),
					CurState::Both((30, 0), (t1, cur1)),
					remaining,
				)?;
				best_score = best_score.max(score);
				remaining.push_back(cur1);
			}
		}
		CurState::Left(t0, cur0) => {
			for _ in 0..remaining.len() {
				let cur1 = remaining.pop_front().unwrap();
				let t1 = travel_times[(prior.1, cur1)] + 1; // + 1 for activating the valve
				let score = find_best_score_in_time_dual(
					travel_times,
					rates,
					time,
					rate,
					(prior.0, prior.1),
					CurState::Both((t0, cur0), (t1, cur1)),
					remaining,
				)?;
				best_score = best_score.max(score);
				remaining.push_back(cur1);
			}
		}
		CurState::Right(t1, cur1) => {
			for _ in 0..remaining.len() {
				let cur0 = remaining.pop_front().unwrap();
				let t0 = travel_times[(prior.0, cur0)] + 1; // + 1 for activating the valve
				let score = find_best_score_in_time_dual(
					travel_times,
					rates,
					time,
					rate,
					(prior.0, prior.1),
					CurState::Both((t0, cur0), (t1, cur1)),
					remaining,
				)?;
				best_score = best_score.max(score);
				remaining.push_back(cur0);
			}
		}
		CurState::Both((t0, cur0), (t1, cur1)) => match t0.cmp(&t1) {
			Ordering::Less => {
				let score = if time + t0 > MAX_DUAL_TIME {
					rate * (MAX_DUAL_TIME as usize - time as usize)
				} else {
					rate * t0 as usize
						+ find_best_score_in_time_dual(
							travel_times,
							rates,
							time + t0,
							rate + rates[cur0].1,
							(cur0, prior.1),
							CurState::Right(t1 - t0, cur1),
							remaining,
						)?
				};
				best_score = best_score.max(score);
			}
			Ordering::Equal => {
				let score = if time + t0 > MAX_DUAL_TIME {
					rate * (MAX_DUAL_TIME as usize - time as usize)
				} else {
					rate * t0 as usize
						+ find_best_score_in_time_dual(
							travel_times,
							rates,
							time + t0,
							rate + rates[cur0].1 + rates[cur1].1,
							(cur0, cur1),
							CurState::Neither,
							remaining,
						)?
				};
				best_score = best_score.max(score);
			}
			Ordering::Greater => {
				let score = if time + t1 > MAX_DUAL_TIME {
					rate * (MAX_DUAL_TIME as usize - time as usize)
				} else {
					rate * t1 as usize
						+ find_best_score_in_time_dual(
							travel_times,
							rates,
							time + t1,
							rate + rates[cur1].1,
							(prior.0, cur1),
							CurState::Left(t0 - t1, cur0),
							remaining,
						)?
				};
				best_score = best_score.max(score);
			}
		},
	}
	Ok(best_score)
}
