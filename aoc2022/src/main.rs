use std::{env, fs};

extern crate core;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

trait Runnable {
    fn run_with_input(&self, input: String);
}
struct Configuration {
    runnable: Box<dyn Runnable>,
    number: u32
}
impl Configuration {
    pub fn run(&self) {
        let input = self.get_input_from_number()
            .expect(format!("Could not find file for day {}", self.number).as_str());
        self.runnable.run_with_input(input);
    }
    fn get_input_from_number(&self) -> std::io::Result<String> {
        let path = format!("inputs/day_{}.txt", self.number.to_string());
        fs::read_to_string(path)
    }
}

fn main() {
    let configurations: Vec<Configuration> = vec![
        Configuration { runnable: Box::new(day_1::Solution), number: 1 },
        Configuration { runnable: Box::new(day_2::Solution), number: 2 },
        Configuration { runnable: Box::new(day_3::Solution), number: 3 },
        Configuration { runnable: Box::new(day_4::Solution), number: 4 },
    ];
    let args: Vec<String> = env::args().collect();
    let selection_args: Vec<&str> = args[1..].into_iter().map(|s| s.as_str()).collect();

    if selection_args.is_empty() {
        // let selection_args = vec!["3"]; // debug runs specific configurations
        configurations.iter().for_each(|c| c.run()); // default runs everything
    }
    else {
        for configuration in &configurations {
            'arg_loop: for selection_arg in &selection_args {
                if selection_arg.contains(configuration.number.to_string().as_str()) {
                    configuration.run();
                    break 'arg_loop;
                }
            }
        }
    }
}
