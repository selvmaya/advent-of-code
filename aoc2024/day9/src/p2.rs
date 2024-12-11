#![doc = include_str!("../p2.md")]

mod common;

use std::fmt::{Debug, Display, Formatter};

use common::*;
use itertools::Itertools;

fn main() {
	util::DayInput::find::<8>().solve_with(solve);
}

/// # Problem
/// For each file (identified by ID) from the right, try and move it as much
/// left as is possible. What is the filesystem checksum then?
/// # Solution
/// Checksum works the same.
/// Instead of moving by position, lets move whole blocks at once.
/// According to the example, we do not need to check for freed up space during
/// compacting.
fn solve(input: impl AsRef<str>) -> u64 {
	let disk_map = input.as_ref().parse::<DiskMap>().expect("parsable");
	eprintln!("Input:    {}", input.as_ref().trim());
	eprintln!("Disk map: {}", disk_map);

	let (mut files, mut free_spaces, _pos): (Vec<File>, Vec<FreeSpace>, _) = disk_map
		.0
		.into_iter()
		.enumerate()
		.map(|(id_n, rest)| (ID(id_n), rest))
		.fold(
			(Vec::new(), Vec::new(), Position(0)),
			|(mut files, mut free_spaces, mut pos), (id, (file_len, free_space_len))| {
				files.push(File {
					index: FileIndex { pos, id },
					len:   file_len
				});
				pos.0 += file_len.0 as usize;
				if let Some(free_space_len) = free_space_len {
					free_spaces.push(FreeSpace {
						index: FreeSpaceIndex { pos },
						len:   free_space_len
					});
					pos.0 += free_space_len.0 as usize;
				}
				//                eprintln!("Alternation complete, pos = {}", pos.0);

				(files, free_spaces, pos)
			}
		);
	debug_assert_eq!(
		_pos.0 as u32,
		input
			.as_ref()
			.trim()
			.chars()
			.map(|c| c.to_digit(10).unwrap())
			.sum(),
		"End position should be known"
	);

	#[allow(clippy::needless_range_loop)]
	for f_index in 0..files.len() {
		let f_copy = files[f_index];
		for s_index in (0..free_spaces.len()).rev() {
			let s_copy = free_spaces[s_index];
			if s_copy.len.0 >= f_copy.len.0 && s_copy.index.pos.0 <= f_copy.index.pos.0 {
				free_spaces[s_index].len.0 -= f_copy.len.0;
				files[f_index].index.pos.0 = s_copy.index.pos.0;
				free_spaces[s_index].index.pos.0 += f_copy.len.0 as usize;
				// note: CANNOT REMOVE it would break everything
				//                if (free_spaces[space_index].len.0 == 0) {
				//                    free_spaces.remove(space_index);
				//                }
				// add new space where file was
				free_spaces.push(FreeSpace {
					index: FreeSpaceIndex {
						pos: f_copy.index.pos
					},
					len:   f_copy.len
				});

				//                leftmost_space.len.0 -= file.len.0; // always
				// >= 0, see above                file.index.pos.0 =
				// leftmost_space.index.pos.0;
				//
				//                leftmost_space.index.pos.0 += file.len.0 as
				// usize;                debug_assert_ne!(
				//                    file.index.pos.0,
				// leftmost_space.index.pos.0,                    "The free
				// space should now start beyond this"                );
			}
		}
	}

	debug_assert!(free_spaces.iter().map(|pos| pos.index.pos.0).all_unique());
	// remove places with empty spaces
	let free_spaces = free_spaces
		.into_iter()
		.filter(|free_space| free_space.len.0 != 0)
		.collect::<Vec<_>>();
	debug_assert!(free_spaces.iter().map(|pos| pos.index.pos.0).all_unique());

	// after moves, the container should reflect the new state
	debug_assert!(files.iter().map(|pos| pos.index.pos.0).all_unique());
	files.sort_by_key(|file| file.index.pos.0);
	debug_assert!(files.iter().map(|pos| pos.index.pos.0).all_unique());

	let merged_data = files
		.into_iter()
		.map(BlockVariant::File)
		// this merge iterator is sorted because free spaces are sorted by definition,
		// and files are sorted above
		.merge_by(free_spaces.into_iter().map(BlockVariant::Space), |a, b| {
			a.pos().0 <= b.pos().0
		})
		.collect::<Vec<_>>();
	let display_data = merged_data.iter().join("");
	eprintln!("Merged: {}", display_data);
	debug_assert_eq!(
		display_data,
		merged_data
			.iter()
			.sorted_by_key(|variant| variant.pos().0)
			.join(""),
		"Merged (left) and sorted (right) should be equal."
	);

	eprintln!();

	debug_assert!(merged_data.is_sorted_by_key(|variant| variant.pos().0));
	let (_, sum) = merged_data
		.into_iter()
		.fold((Position(0), 0u64), |(mut pos, mut sum), next| {
			match next {
				BlockVariant::File(file) => {
					eprintln!(
						"Adding file {} at {} to checksum, and also counting it (len={})",
						file, pos.0, file.len.0
					);
					debug_assert_eq!(pos.0, file.index.pos.0); // this should be synchronized
					for _ in 0..file.len.0 {
						// we multiply with the pos state and not the beginning of the file
						sum += pos.0 as u64 * file.index.id.0 as u64;
						pos.0 += 1;
					}
				},
				BlockVariant::Space(space) => {
					eprintln!(
						"Counting space {} (len={}) at pos {}",
						space, space.len.0, pos.0
					);
					debug_assert_eq!(pos.0, space.index.pos.0); // this should be synchronized
					pos.0 += space.len.0 as usize;
				}
			}
			(pos, sum)
		});
	sum

	//    todo!()

	//    let sum = solve_a(input, disk_map);
	//    sum
}

#[expect(unused)]
fn solve_a(mut files: Vec<File>, mut free_spaces: Vec<FreeSpace>) -> u64 {
	let merged_data = files
		.iter()
		.copied()
		.map(BlockVariant::File)
		// this merge iterator is sorted because free spaces are sorted by definition,
		// and files are sorted above
		.chain(free_spaces.iter().copied().map(BlockVariant::Space))
		.sorted_by_key(|variant| variant.pos().0)
		.collect::<Vec<_>>();
	eprintln!("Pre-compact: {}\n", merged_data.iter().join(""));

	// move files from the right into free spaces to the left, as fast as possible
	for file in files.iter_mut().rev() {
		// todo: observe that free spaces do not get put back
		// only move file if a space exists that is to the left and has enough space
		if let Some(leftmost_space) = free_spaces.iter_mut().find(|free_space| {
			free_space.index.pos.0 < file.index.pos.0 && free_space.len.0 >= file.len.0
		}) {
			eprintln!(
				"Moving `{}` inside `{}` ({}->{}, since len {} <= {})",
				file,
				leftmost_space,
				file.index.pos.0,
				leftmost_space.index.pos.0,
				file.len.0,
				leftmost_space.len.0
			);
			leftmost_space.len.0 -= file.len.0; // always >= 0, see above
									   // notice that they are never removed.
									   // Their existence in the vec is necessary for the next step.
									   // this file is now located in the left area of the (now previously) free space
			eprintln!(" * remaining len for space: {}", leftmost_space.len.0);
			file.index.pos.0 = leftmost_space.index.pos.0;

			// this is important because other files should be at a different spot if there
			// is more space left
			leftmost_space.index.pos.0 += file.len.0 as usize;
			debug_assert_ne!(
				file.index.pos.0, leftmost_space.index.pos.0,
				"The free space should now start beyond this"
			);
			eprintln!(" * new pos for space: {}", leftmost_space.index.pos.0);
		} else {
			eprintln!("Could not move file {} to a free space", file);
		}
	}
	eprintln!();

	debug_assert!(free_spaces.iter().map(|pos| pos.index.pos.0).all_unique());
	// remove places with empty spaces
	let free_spaces = free_spaces
		.into_iter()
		.filter(|free_space| free_space.len.0 != 0)
		.collect::<Vec<_>>();
	debug_assert!(free_spaces.iter().map(|pos| pos.index.pos.0).all_unique());

	// after moves, the container should reflect the new state
	debug_assert!(files.iter().map(|pos| pos.index.pos.0).all_unique());
	files.sort_by_key(|file| file.index.pos.0);
	debug_assert!(files.iter().map(|pos| pos.index.pos.0).all_unique());

	let merged_data = files
		.into_iter()
		.map(BlockVariant::File)
		// this merge iterator is sorted because free spaces are sorted by definition,
		// and files are sorted above
		.merge_by(free_spaces.into_iter().map(BlockVariant::Space), |a, b| {
			a.pos().0 <= b.pos().0
		})
		.collect::<Vec<_>>();
	let display_data = merged_data.iter().join("");
	eprintln!("Merged: {}", display_data);
	debug_assert_eq!(
		display_data,
		merged_data
			.iter()
			.sorted_by_key(|variant| variant.pos().0)
			.join(""),
		"Merged (left) and sorted (right) should be equal."
	);

	eprintln!();

	debug_assert!(merged_data.is_sorted_by_key(|variant| variant.pos().0));
	let (_, sum) = merged_data
		.into_iter()
		.fold((Position(0), 0u64), |(mut pos, mut sum), next| {
			match next {
				BlockVariant::File(file) => {
					eprintln!(
						"Adding file {} at {} to checksum, and also counting it (len={})",
						file, pos.0, file.len.0
					);
					debug_assert_eq!(pos.0, file.index.pos.0); // this should be synchronized
					for _ in 0..file.len.0 {
						// we multiply with the pos state and not the beginning of the file
						sum += pos.0 as u64 * file.index.id.0 as u64;
						pos.0 += 1;
					}
				},
				BlockVariant::Space(space) => {
					eprintln!(
						"Counting space {} (len={}) at pos {}",
						space, space.len.0, pos.0
					);
					debug_assert_eq!(pos.0, space.index.pos.0); // this should be synchronized
					pos.0 += space.len.0 as usize;
				}
			}
			(pos, sum)
		});
	sum
}

#[derive(Debug, Copy, Clone)]
enum BlockVariant {
	File(File),
	Space(FreeSpace)
}
impl Display for BlockVariant {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			BlockVariant::File(file) => Display::fmt(file, f),
			BlockVariant::Space(space) => Display::fmt(space, f)
		}
	}
}
impl BlockVariant {
	fn pos(&self) -> &Position {
		match self {
			BlockVariant::File(file) => &file.index.pos,
			BlockVariant::Space(space) => &space.index.pos
		}
	}
}

#[derive(Debug, Copy, Clone)]
struct Block<I> {
	index: I,
	len:   BlockLen
}
type File = Block<FileIndex>;
type FreeSpace = Block<FreeSpaceIndex>;
impl Display for File {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for _ in 0..self.len.0 {
			write!(f, "{}", self.index.id.0)?;
		}
		Ok(())
	}
}
impl Display for FreeSpace {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for _ in 0..self.len.0 {
			write!(f, ".")?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn example_solvable() {
		assert_eq!(super::solve(include_str!("EXAMPLE")), 2858);
	}

	#[ignore]
	#[test]
	fn input_solvable() {
		assert_eq!(super::solve(include_str!("../../inputs/9")), 0);
	}
}
