#![allow(unused)]

/// Tries finding the relevant input, either by accepting a string or falling
/// back to some maybe-existing file.
/// The `inputs` folder may be a private submodule.
pub fn day_input<const DAY: u8>() -> &'static str {
	let relative_path = std::env::args().nth(1).unwrap_or_else(|| {
		let default = format!("inputs/{}", DAY);
		eprintln!("No path specified, assuming you want '{}'.", default);
		default
	});
	let absolute = std::path::absolute(relative_path).expect("absolute path can be put together");
	eprintln!("Looking for {:?}...", absolute);
	std::fs::read_to_string(absolute)
		.expect("file needs to exist")
		.leak()
}

// dev-only prints
#[cfg(debug_assertions)]
macro_rules! dprintln {
	($x:expr) => {
		eprintln!($x);
	};
}
#[cfg(not(debug_assertions))]
macro_rules! dprintln {
	($x:expr) => {}; // no-op
}
pub(crate) use dprintln;
