use std::{
	fmt::{Display, Formatter},
	str::FromStr
};

#[allow(unused_imports)]
use winnow::{
	ascii::*,
	combinator::*,
	error::*,
	token::*,
	{PResult, Parser}
};

#[derive(Debug, Copy, Clone)]
pub struct BlockLen(pub u8);
#[derive(Debug, Copy, Clone)]
pub struct ID(pub usize);
#[derive(Debug, Copy, Clone)]
pub struct Position(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct FileIndex {
	pub pos: Position,
	pub id:  ID
}
#[derive(Debug, Copy, Clone)]
pub struct FreeSpaceIndex {
	pub pos: Position
}

pub struct DiskMap(pub Vec<(BlockLen, Option<BlockLen>)>);
impl Display for DiskMap {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for (file_block, free_space_block) in &self.0 {
			write!(f, "{}", file_block.0)?;
			if let Some(free_space_block) = free_space_block {
				write!(f, "{}", free_space_block.0)?;
			}
		}
		Ok(())
	}
}
impl FromStr for DiskMap {
	type Err = ErrMode<ContextError>;

	fn from_str(mut s: &str) -> Result<Self, Self::Err> {
		parse_disk_map.parse_next(&mut s)
	}
}

/// Input is a single line of digits.
/// First digit is a file block, second is a free space block. These alternate
/// repeatedly.
fn parse_disk_map(input: &mut &str) -> PResult<DiskMap> {
	repeat(1.., (parse_block, opt(parse_block)))
		.parse_next(input)
		.map(DiskMap)
}
fn parse_block(input: &mut &str) -> PResult<BlockLen> {
	take(1usize).parse_to().map(BlockLen).parse_next(input)
}
