use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        part_1(&input);
        part_2(&input);
    }
}

// part 1 //

// rucksack has 2 large compartments
#[derive(Debug)]
struct Rucksack {
    compartments: [Compartment; 2]
}

// all items go into exactly 1 of the 2 compartments
#[derive(Debug)]
struct Compartment {
    items: Vec<Item>
}

// the error is that one item per backpack is in the wrong compartment

// we have a list of all items in each rucksack (puzzle input)

// each item "type" is identified by a single letter (upper or lower-case)
#[derive(PartialEq, Debug)]
struct Item {
    letter_type: char
}


// input (list of items) are given as characters on a single line
// each compartment has exactly the same amount of items,
// and the input is split into each compartment in a rucksack through the middle
impl Rucksack {
    fn from_str(str_input: &str) -> Self {
        let split_point = str_input.len() / 2;
        let str_compartments = str_input.split_at(split_point);
        Rucksack {
            compartments: [
                Compartment::from_str(str_compartments.0),
                Compartment::from_str(str_compartments.1)
            ]
        }
    }
}
impl Compartment {
    fn from_str(str_input: &str) -> Self {
        Compartment {
            items: str_input.chars().map( | c| Item { letter_type: c }).collect()
        }
    }
}

// every item type can be converted to a "priority" (number)
// priorities are 1-26 (lower case) and 27-52 (upper case)
const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];
impl Item {
    fn get_priority(&self) -> Result<u32, String> {
        let letter = self.letter_type;
        match ASCII_LOWER.iter().position(|&c| c == letter) {
            Some(p) => Ok(p as u32 + 1),
            None => {
                let letter_as_lower = letter.to_ascii_lowercase();
                match ASCII_LOWER.iter().position(|&c| c == letter_as_lower) {
                    Some(p) => Ok(p as u32 + 27),
                    None => Err(format!("Could not find priority of letter '{}'", letter))
                }
            }
        }
    }
}

// find the item that appears in both compartments
impl Rucksack {
    fn find_duplicate(&self) -> Option<&Item> {
        self.compartments[0].items.iter().find(|&item| self.compartments[1].items.contains(item))
    }
}

// what is the sum of the priorities (number value) of the duplicate item types?

#[allow(unused)]
fn part_1(input: &str) {
    let rucksacks: Vec<Rucksack> = input.lines()
        .map(Rucksack::from_str).collect();

    let priority_sum: u32 = rucksacks.iter()
        .map(|r| r.find_duplicate()
            .unwrap_or_else(|| panic!("Rucksack '{:?}' does not contain any duplicates", r)))
        .map(|item| item.get_priority().unwrap())
        .sum();

    println!("Sum of priority for all duplicate items: {}", priority_sum);
}

// part 2 //

// elves are divided into groups of 3
#[derive(Debug)]
struct Group {
    elves: [Elf; 3]
}

// each elf has 1 rucksack
#[derive(Debug)]
struct Elf {
    rucksack: Rucksack
}

// each group has 1 "badge" that is an item with a specific letter, which we do not know
// we need to find all the badges of each group, by seeing which item all elves in the group has
impl Rucksack {
    fn contains(&self, item: &Item) -> bool {
        for compartment in &self.compartments {
            if compartment.items.contains(item) {
                return true;
            }
        }
        false
    }
    fn all_items(&self) -> Vec<&Item> {
        let mut items: Vec<&Item> = Vec::new();
        for compartment in &self.compartments {
            for item in &compartment.items {
                items.push(item);
            }
        }
        items
    }
}
impl Group {
    fn find_badge(&self) -> Option<&Item> {
        self.elves[0].rucksack.all_items().into_iter().find(|&item| self.elves[1].rucksack.contains(item) && self.elves[2].rucksack.contains(item))
    }
}

// each three lines in input corresponds to a single group of 3 elves
impl Group {
    fn from_str_rucksacks(str_rucksacks: [&str; 3]) -> Self {
        Group { elves: str_rucksacks.map(|r_s|
            Elf { rucksack: Rucksack::from_str(r_s) })
        }
    }
}

// what is the sum of the priorities (number values) for the badge of every group?

#[allow(unused)]
fn part_2(input: &str) {
    // gather all groups
    let mut groups: Vec<Group> = Vec::new();
    let mut str_rucksacks: Vec<&str> = Vec::new();
    for line in input.lines() {
        str_rucksacks.push(line);
        if str_rucksacks.len() == 3 {
            groups.push(
                Group::from_str_rucksacks(
                    str_rucksacks[..].try_into()
                        .unwrap_or_else(|_| panic!("Group starting with rucksack {} could not be converted to array", &str_rucksacks[0]))));
            str_rucksacks = Vec::new();
        }
    }

    // find badges of all groups
    let badges: Vec<&Item> = groups.iter().map(|g| g.find_badge()
        .unwrap_or_else(|| panic!("Group '{:?}' did not have a common badge", g)))
        .collect();

    // display sum of "priorities" (values) for each badge
    let sum: u32 = badges.iter().map(|b| b.get_priority().unwrap()).sum();
    println!("Sum of priorities for badge of each group: {}", sum)
}
