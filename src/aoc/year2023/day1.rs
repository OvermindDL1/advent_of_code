use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::bail;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "calibration data"
	#[clap(default_value_t = DataFrom::internal(2023, 1))]
	pub input: DataFrom,
}

impl Day1 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i32, i32)> {
		let (step1, step2) = fold_trimmed_nonempty_lines_of_file_bytes(
			&self.input,
			(0i32, 0i32),
			|(acc1, acc2), line| {
				let step1 = {
					let first = line
						.iter()
						.copied()
						.find(u8::is_ascii_digit)
						.ok_or_else(|| {
							anyhow::anyhow!(
								"No number found in line: {:?}",
								std::str::from_utf8(line)
							)
						})?;
					let last = line
						.iter()
						.copied()
						.rev()
						.find(u8::is_ascii_digit)
						.ok_or_else(|| {
							anyhow::anyhow!(
								"No number found in line: {:?}",
								std::str::from_utf8(line)
							)
						})?;
					(first - b'0') as i32 * 10 + (last - b'0') as i32
				};

				let step2 = {
					// Eh, regex would be easier, but this fun and maybe even faster?
					let first = {
						let mut bytes = line;
						loop {
							if bytes.is_empty() {
								bail!("No number found in line: {:?}", std::str::from_utf8(line));
							}
							match bytes {
								[b'0', ..] | [b'z', b'e', b'r', b'o', ..] => break 00,
								[b'1', ..] | [b'o', b'n', b'e', ..] => break 10,
								[b'2', ..] | [b't', b'w', b'o', ..] => break 20,
								[b'3', ..] | [b't', b'h', b'r', b'e', b'e', ..] => break 30,
								[b'4', ..] | [b'f', b'o', b'u', b'r', ..] => break 40,
								[b'5', ..] | [b'f', b'i', b'v', b'e', ..] => break 50,
								[b'6', ..] | [b's', b'i', b'x', ..] => break 60,
								[b'7', ..] | [b's', b'e', b'v', b'e', b'n', ..] => break 70,
								[b'8', ..] | [b'e', b'i', b'g', b'h', b't', ..] => break 80,
								[b'9', ..] | [b'n', b'i', b'n', b'e', ..] => break 90,
								_ => bytes = &bytes[1..],
							}
						}
					};
					let last = {
						let mut bytes = line;
						loop {
							if bytes.is_empty() {
								bail!("No number found in line: {:?}", std::str::from_utf8(line));
							}
							match bytes {
								[.., b'0'] | [.., b'z', b'e', b'r', b'o'] => break 0,
								[.., b'1'] | [.., b'o', b'n', b'e'] => break 1,
								[.., b'2'] | [.., b't', b'w', b'o'] => break 2,
								[.., b'3'] | [.., b't', b'h', b'r', b'e', b'e'] => break 3,
								[.., b'4'] | [.., b'f', b'o', b'u', b'r'] => break 4,
								[.., b'5'] | [.., b'f', b'i', b'v', b'e'] => break 5,
								[.., b'6'] | [.., b's', b'i', b'x'] => break 6,
								[.., b'7'] | [.., b's', b'e', b'v', b'e', b'n'] => break 7,
								[.., b'8'] | [.., b'e', b'i', b'g', b'h', b't'] => break 8,
								[.., b'9'] | [.., b'n', b'i', b'n', b'e'] => break 9,
								_ => bytes = &bytes[..bytes.len() - 1],
							}
						}
					};
					first + last
				};

				Ok((acc1 + step1, acc2 + step2))
			},
		)?;

		Ok((step1, step2))
	}
}
