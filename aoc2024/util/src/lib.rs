#![allow(unused)]

/// Input that a solver can take.
pub struct DayInput<S: AsRef<str>>(S);
impl DayInput<String> {
    /// Tries finding the relevant input, either by accepting a string or
    /// falling back to some maybe-existing file.
    /// The `inputs` folder may be a private submodule.
    pub fn find<const DAY: u8>() -> Self {
        let absolute = {
            let relative_path = std::env::args().nth(1).unwrap_or_else(|| {
                let default = format!("inputs/{}", DAY);
                eprintln!("No path specified, assuming you want '{}'.", default);
                default
            });
            std::path::absolute(&relative_path).expect("absolute path creation")
        };
        eprintln!("Looking for {:?}...", absolute);
        let x = std::fs::read_to_string(absolute).expect("file needs to exist");
        eprintln!("Found file! Returning it to the program.\n");
        DayInput(x)
    }

    // elided lifetimes gives problems where function is not general enough
    #[allow(clippy::needless_lifetimes)]
    pub fn solve_with<'s, O: Display>(&'s self, solver: impl FnOnce(&'s Self) -> O) {
        let o = solver(self);
        println!("Result: {}", o);
    }
}
impl AsRef<str> for DayInput<String> {
    fn as_ref(&self) -> &str {
        &self.0
    }
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

use std::fmt::Display;

pub(crate) use dprintln;
