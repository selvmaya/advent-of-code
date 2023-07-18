use std::{env, fs};

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

trait Runnable {
	fn run_with_input(&self, input: String);
}
struct Configuration {
	runnable: Box<dyn Runnable>,
	number:   u32
}
impl Configuration {
	fn run(&self, should_run_test: bool) {
		self.run_with_input(if should_run_test {
			self.get_test_input_from_number()
				.unwrap_or_else(|_| panic!("*TEST* input missing for day {}", self.number))
		} else {
			self.get_input_from_number()
				.unwrap_or_else(|_| panic!("Input missing for day {}", self.number))
		})
	}

	fn run_with_input(&self, input: String) {
		println!("--- RUNNING DAY {} ---", self.number);
		self.runnable.run_with_input(input);
	}

	fn get_input_from_number(&self) -> std::io::Result<String> {
		let path = format!("inputs/day_{}.txt", self.number);
		fs::read_to_string(path)
	}

	fn get_test_input_from_number(&self) -> std::io::Result<String> {
		let path = format!("inputs/day_{}_test.txt", self.number);
		fs::read_to_string(path)
	}
}

fn main() {
	let all_configurations = [
		Configuration {
			runnable: Box::new(day_1::Solution),
			number:   1
		},
		Configuration {
			runnable: Box::new(day_2::Solution),
			number:   2
		},
		Configuration {
			runnable: Box::new(day_3::Solution),
			number:   3
		},
		Configuration {
			runnable: Box::new(day_4::Solution),
			number:   4
		},
		Configuration {
			runnable: Box::new(day_5::Solution),
			number:   5
		},
		Configuration {
			runnable: Box::new(day_6::Solution),
			number:   6
		},
		Configuration {
			runnable: Box::new(day_7::Solution),
			number:   7
		},
		Configuration {
			runnable: Box::new(day_8::Solution),
			number:   8
		},
		Configuration {
			runnable: Box::new(day_9::Solution),
			number:   9
		},
		Configuration {
			runnable: Box::new(day_10::Solution),
			number:   10
		},
		Configuration {
			runnable: Box::new(day_11::Solution),
			number:   11
		},
		Configuration {
			runnable: Box::new(day_12::Solution),
			number:   12
		},
		Configuration {
			runnable: Box::new(day_13::Solution),
			number:   13
		}
	];
	let mut args = env::args();
	args.next(); // discard first element since it isn't user-relevant

	let selection_arg = args.next(); // argument 1 determines what configuration to run
	let try_run_as_test = match args.next() {
		// argument 2 determines if it should try and use test input
		Some(val) => matches!(val.as_str(), "test"),
		_ => false
	};

	if let Some(selection) = selection_arg {
		// run a specific configuration
		for configuration in &all_configurations {
			if selection.as_str() == configuration.number.to_string().as_str() {
				configuration.run(try_run_as_test);
				println!();
			}
		}
	} else {
		// default runs everything
		all_configurations.iter().for_each(|c| {
			c.run_with_input(c.get_input_from_number().expect("Could not find input!"))
		});
	}
}
