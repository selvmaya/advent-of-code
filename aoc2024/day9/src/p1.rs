#![doc = include_str!("../p1.md")]

mod common;
use common::*;

use itertools::{EitherOrBoth, Itertools};
use winnow::Parser;

fn main() {
    util::DayInput::find::<9>().solve_with(solve);
}

/// # Input
/// A disk map (digits) representing the layout of files and free space.
/// The digits alternate between being the length of files and the length of free space.
/// There is no inherent file data here, only the map of the data.
/// # Problem
/// Calculate the filesystem checksum of the compacted version of the given disk map.
/// ## Checksum
/// The sum of each file block ID times its position.
/// ## ID
/// The order of the file blocks before compacting (every file should know this).
/// IDs can be more than one digit.
/// ## Position
/// The position of the files after compacting. This can also be more than one digit.
/// Positions are in the last line of the compacting example one digit per position.
/// ## Compacting
/// Take the rightmost file (in the example a single digit, `9`),
/// and move it to the leftmost free space. Continue until every free space is on the right side
/// of all the files (visible as numbers in the example). IDs do not take up space, they are
/// metadata. Every file takes up equal space, equal to one free space.
/// # Solution
/// ## Data
/// Free spaces between numbers can be a Vec, which we can pop elements from when compacting.
/// If it is empty we are done compacting.
/// Replacing elements on the left with *different* elements on the right, can be achieved using
/// (zipped) iterators. We want to avoid the cost of inserting/shuffling elements around in a Vec
/// of files, so we can swap in place or just not build a compacted Vec at all.
/// If the free spaces know their position, we can just take files from the right.
/// Technically we don't even need to build a final representation, we can just calculate the sum
/// by figuring out the result for each position. Order is irrelevant outside of this value.
/// Separating files from free space can be done by unzipping an iterator/collection of tuples.
/// The only problem with this is maintaining their positions. We can do this with a fold that
/// operates on both types of values and generates a collection/iterator with each position
/// yielded/updated as it happens.
/// ## Parsing
/// If we manage to separate files from free space, getting file IDs is a trivial `enumerate`-operation.
/// We are allowed to separate the files from the free space as long as both know their original
/// position, so that "swapping" (or calculating as-if) has the right effect in the result.
fn solve(input: impl AsRef<str>) -> u64 {
    let disk_map = input.as_ref().parse::<DiskMap>().expect("parsable");
    let (files, file_spaces, _) = disk_map.0
            .into_iter()
            // enumerating here gives us file (block) IDs
            .enumerate()
            .map(|(id_n, rest)| (ID(id_n), rest))
            .fold(
        (Vec::new(), Vec::new(), Position(0)),
        |(mut files, mut free_space, mut shared_pos): (Vec<FileIndex>, Vec<FreeSpaceIndex>, Position),
         (id, (file_block_len, free_space_len)): (ID, (BlockLen, Option<BlockLen>))| {
            for _ in 0..file_block_len.0 {
                files.push(FileIndex { pos: shared_pos, id });
                shared_pos.0 += 1;
            }
            // if the total block count is odd,
            // there is no free space block to go along with the last file block.
            if let Some(free_space_len) = free_space_len {
                for _ in 0..free_space_len.0 {
                    free_space.push(FreeSpaceIndex { pos: shared_pos });
                    shared_pos.0 += 1;
                }
            }

            (files, free_space, shared_pos)
        },
    );

    // both vectors semantically go from left to right until file iterator is reversed
    file_spaces
        .into_iter()
        .zip_longest(files.into_iter().rev())
        .map_while(|a| match a {
            // file as well as a free space to the left of it
            EitherOrBoth::Both(leftmost_free_space, rightmost_file)
                if leftmost_free_space.pos.0 < rightmost_file.pos.0 =>
            {
                Some((leftmost_free_space.pos.0 * rightmost_file.id.0) as u64)
            }
            // file but no more free spaces further left of it
            EitherOrBoth::Right(rightmost_file) | EitherOrBoth::Both(_, rightmost_file) => {
                Some((rightmost_file.pos.0 * rightmost_file.id.0) as u64)
            }
            // free space but no more files to move [ends]
            EitherOrBoth::Left(_leftmost_free_space) => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 1928);
    }

    //    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/9")), 0);
    }
}
