pub(crate) use {part_1::part_1, part_2::part_2};

mod part_1 {
    use super::*;
    use itertools::Itertools;
    use std::collections::HashMap;
    use std::error::Error;
    use std::process::id;
    use std::str::FromStr;
    use std::sync::Arc;
    use strum::{Display, EnumString};

    #[derive(Display, EnumString, Copy, Clone)]
    enum Instruction {
        // direction to walk
        #[strum(serialize = "L")]
        Left,
        #[strum(serialize = "R")]
        Right,
    }

    #[derive(Hash, Debug, Eq, PartialEq, Clone)]
    struct ID(Arc<[char; 3]>);
    impl FromStr for ID {
        type Err = Box<dyn Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(Arc::new(
                s.chars().collect::<Vec<_>>().as_slice().try_into()?,
            )))
        }
    }

    #[derive(Debug)]
    struct Branches(ID, ID);
    impl Branches {
        fn select(&self, instruction: Instruction) -> ID {
            match instruction {
                Instruction::Left => self.0.clone(),
                Instruction::Right => self.1.clone(),
            }
        }
    }
    impl FromStr for Branches {
        type Err = Box<dyn Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (left, right) = s
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .ok_or(format!("Could not split {} into two IDs", s))?;
            Ok(Branches(left.parse()?, right.parse()?))
        }
    }

    /// Calculate how many steps it takes to reach value ZZZ.
    /// Instruction steps are repeated until value is reached.
    /// Steps are defined as node jumps in the graph defined by input.
    /// We do not need to worry about infinite graph cycle,
    /// as the input should be guaranteed against a truly infinite loop.
    pub(crate) fn part_1(input: &str) -> String {
        let source_id = ID(Arc::new(['A', 'A', 'A']));
        let target_id = ID(Arc::new(['Z', 'Z', 'Z']));

        let mut lines = input.lines();

        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();
        lines.next().unwrap(); // discard empty line

        let graph = lines
            .map(|line| {
                let (source, targets) = line.split_once(" = ").unwrap();
                (
                    source.parse().unwrap(),
                    targets.parse::<Branches>().unwrap(),
                )
            })
            .collect::<HashMap<ID, Branches>>();
        let mut instruction_cycle = instructions.into_iter().cycle();
        let mut id = source_id;
        let mut steps = 0;
        eprintln!("Starting at step {}: {:?}", steps, id);
        loop {
            steps += 1;
            let instruction = instruction_cycle.next().unwrap();
            id = graph.get(&id).unwrap().select(instruction);
            if id == target_id {
                eprintln!("TARGET FOUND at step {}: {:?}", steps, id);
                break steps; // one more, but is OK bc enumerate starts at zero
            }
        }
        .to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        #[test]
        fn sample_works() {
            assert_eq!(part_1(TEST_INPUT), "2");
        }

        #[test]
        fn example_works() {
            const EXAMPLE_INPUT: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
            assert_eq!(part_1(EXAMPLE_INPUT), "6");
        }
    }
}

mod part_2 {
    use super::*;
    use itertools::Itertools;
    use std::collections::{BTreeSet, HashMap};
    use std::error::Error;
    use std::process::id;
    use std::str::FromStr;
    use std::sync::Arc;
    use strum::{Display, EnumString};

    #[derive(Display, EnumString, Copy, Clone)]
    enum Instruction {
        // direction to walk
        #[strum(serialize = "L")]
        Left,
        #[strum(serialize = "R")]
        Right,
    }

    #[derive(Hash, Debug, Eq, PartialEq, Clone)]
    struct Node(Arc<[char; 3]>);

    impl FromStr for Node {
        type Err = Box<dyn Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(Arc::new(
                s.chars().collect::<Vec<_>>().as_slice().try_into()?,
            )))
        }
    }

    #[derive(Debug)]
    struct Branches(Node, Node);

    impl Branches {
        fn select(&self, instruction: &Instruction) -> &Node {
            match instruction {
                Instruction::Left => &self.0,
                Instruction::Right => &self.1,
            }
        }
    }

    impl FromStr for Branches {
        type Err = Box<dyn Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (left, right) = s
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .ok_or(format!("Could not split {} into two IDs", s))?;
            Ok(Branches(left.parse()?, right.parse()?))
        }
    }

    /// Calculate how many steps it takes to reach value ZZZ.
    /// Instruction steps are repeated until value is reached.
    /// Steps are defined as node jumps in the graph defined by input.
    /// We do not need to worry about infinite graph cycle,
    /// as the input should be guaranteed against a truly infinite loop.
    ///
    /// Idea for optimization:
    /// Repeating cycles can be noticed and kept track off.
    /// Each of these repeating cycles can be traversed by simple calculations,
    /// instead of hashing/pointer stepping.
    /// Each traversal (id of ids) can be compared with the others
    /// to find a time when they end up on the winning spot at a winning spot at the same time
    pub(crate) fn part_2(input: &str) -> String {
        let mut lines = input.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();
        lines.next().unwrap(); // discard empty line
        let graph = lines
            .map(|line| {
                let (source, targets) = line.split_once(" = ").unwrap();
                (
                    source.parse().unwrap(),
                    targets.parse::<Branches>().unwrap(),
                )
            })
            .collect::<HashMap<Node, Branches>>();
        let mut instruction_cycle = instructions.into_iter().enumerate().cycle();
        struct Path {
            current_location: Node,
            visited_locations_instruction_indices: HashMap<Node, BTreeSet<usize>>,
        }
        let mut paths: Vec<Path> = graph
            .keys()
            .filter(|key| key.0[2] == 'A') // last thing is A
            .map(|id| Path {
                current_location: id.clone(),
                visited_locations_instruction_indices: HashMap::from([(
                    id.clone(),
                    BTreeSet::new(),
                )]),
            }) // BTreeSet for remembering visited spaces
            .collect();
        struct PathCycle {
            offset: usize,
            cycle_length: usize,
            z_nodes_in_cycle: Vec<usize>,
        }
        impl PathCycle {
            fn get_cycle_index(&self, index: usize) -> Option<usize> {
                Some((index.checked_sub(self.offset))? % self.cycle_length)
            }
            fn can_end_on(&self, index: usize) -> bool {
                if let Some(cycle_index) = self.get_cycle_index(index) {
                    self.z_nodes_in_cycle
                        .iter()
                        .any(|node_index| *node_index == cycle_index)
                } else {
                    false
                }
            }
        }
        // let mut cycles = Vec::new();
        for (steps, (instruction_index, instruction)) in instruction_cycle.enumerate() {
            let mut completed = Vec::new();
            for (path_index, path) in paths.iter_mut().enumerate() {
                path.current_location = graph
                    .get(&path.current_location)
                    .unwrap()
                    .select(&instruction)
                    .clone();
                let not_cycling = path
                    .visited_locations_instruction_indices
                    .entry(path.current_location.clone())
                    .or_default()
                    .insert(instruction_index);
                if !not_cycling {
                    completed.push(path_index);
                }
            }
            for index in completed.iter().sorted().rev() {
                // cycles.push(paths.remove(*index);
            }
            if paths.is_empty() {
                break;
            }
        }
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        #[test]
        #[ignore]
        fn part_2_works() {
            assert_eq!(part_2(TEST_INPUT), "6");
        }
    }
}