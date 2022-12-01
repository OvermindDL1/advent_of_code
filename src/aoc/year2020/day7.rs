use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use petgraph::prelude::*;
use smol_str::SmolStr;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Parser)]
pub struct Day7 {
	/// The input file to use with the parseable rules
	#[clap(default_value_t = DataFrom::Internal {year: 2020, day: 7})]
	pub input: DataFrom,
}

impl Day7 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut rules_graph = Graph::new();
		let mut rules = HashMap::with_capacity(1024);
		process_trimmed_nonempty_lines_of_file(&self.input, |line| {
			let (this_bag, can_contain) = line
				.split_once(" bags contain ")
				.context("invalid bag rule specifier")?;
			let node = rules_graph.add_node(SmolStr::new(this_bag));
			let can_contain: Vec<_> = can_contain
				.trim_end_matches('.')
				.split(", ")
				.filter(|&can_contain| can_contain != "no other bags")
				.map(|can_contain| {
					let (count, can_contain) = can_contain
						.trim_end_matches('s')
						.trim_end_matches(" bag")
						.split_once(' ')
						.with_context(|| {
							format!("invalid bag format in contains section: {}", can_contain)
						})?;
					Ok((count.parse::<usize>()?, SmolStr::new(can_contain)))
				})
				.collect::<anyhow::Result<_>>()?;
			rules.insert(SmolStr::new(this_bag), (node, can_contain));
			Ok(())
		})?;

		for (this_node, can_contain) in rules.values() {
			for (count, can_contain) in can_contain {
				let other_node = &rules[can_contain].0;
				rules_graph.add_edge(*this_node, *other_node, *count);
			}
		}

		{
			let mut possible_external_colors = HashSet::with_capacity(rules.len());
			let mut to_process = Vec::with_capacity(rules.len());
			to_process.push(rules["shiny gold"].0);
			while let Some(next_node) = to_process.pop() {
				for node in rules_graph.neighbors_directed(next_node, Incoming) {
					if possible_external_colors.insert(node) {
						to_process.push(node);
					}
				}
			}
			println!("Step 1: {}", possible_external_colors.len());
		}

		{
			let mut total_bags = HashMap::with_capacity(rules.len());
			let mut to_process = Vec::with_capacity(rules.len());
			to_process.push((rules["shiny gold"].0, 1));
			while let Some((next_node, mult)) = to_process.pop() {
				for edge in rules_graph.edges_directed(next_node, Outgoing) {
					*total_bags.entry(edge.target()).or_default() += *edge.weight() * mult;
					to_process.push((edge.target(), *edge.weight() * mult));
				}
			}
			println!("Step 1: {}", total_bags.values().sum::<usize>());
		}

		Ok(())
	}
}
