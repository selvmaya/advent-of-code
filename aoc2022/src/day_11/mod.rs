use itertools::Itertools;

use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!("PART 1:\n{}\n", part_1_solve(&input));
        println!("PART 2:\n{}\n", part_2_solve(&input));
    }
}

/// monkey activities are noted by puzzle input.
/// rounds will go turn by turn (by number/order), one monkey at a time.
/// each monkey's turn, they will inspect their item then throw it to another monkey.
///
/// items are noted by their worry level/value, assumed to be a positive integer.
/// on inspection, an item will have their value changed by an "operation" (multiply or add) with a number,
/// then before passing the item on, the worry level will be divided by 3 (rounded down to integer).
///
/// "monkey business" is the amount of inspected items by the two most active monkeys multiplied together.
///
/// SOLVE: the level of "monkey business" after 20 rounds
fn part_1_solve(input: &str) -> u64 {
    let monkeys = &mut create_monkeys(input)[..];

    // println!("At the start, the monkeys are holding these worry levels:");
    // for (id, monkey) in monkeys.iter().enumerate() {
    //     println!("Monkey {}: {}", id, monkey.items.iter().join(", "));
    // }
    // println!();

    for _round in 1..=20 {
        for monkey_id in 0..monkeys.len() {
            let monkey_ref = &monkeys[monkey_id];

            // println!("Monkey {}:", monkey_id);
            let new_items: Vec<(MonkeyID, Item)> = monkey_ref
                .items
                .iter()
                .map(|item| {
                    // println!("  Monkey inspects an item with a worry level of {}", item);
                    let value = match monkey_ref.inspection.value {
                        Value::SelfReferential => *item,
                        Value::Specific(value) => value,
                    };
                    let post_inspection = match monkey_ref.inspection.operation {
                        Operation::Multiply => {
                            // print!("    Worry level is multiplied by ");
                            item * value
                        }
                        Operation::Add => {
                            // print!("    Worry level increases by ");
                            item + value
                        }
                    };
                    // println!("{} to {}", value, post_inspection);

                    let post_worry = post_inspection / 3;
                    // println!("    Monkey gets bored with item. Worry level is divided by 3 to {}", post_worry);
                    let target_id = if post_worry % monkey_ref.test.0 == 0 {
                        // println!("    Current worry level is divisible by {}.", monkey_ref.test.0);
                        monkey_ref.outcome.true_target
                    } else {
                        // println!("    Current worry level is not divisible by {}.", monkey_ref.test.0);
                        monkey_ref.outcome.false_target
                    };
                    // println!("    Item with worry level {} is (will be) thrown to monkey {}.", post_worry, target_id);
                    (target_id, post_worry)
                })
                .collect();

            monkeys[monkey_id].inspection.performed += monkey_ref.items.len() as u64;
            monkeys[monkey_id].items.clear();

            for (target_id, item) in new_items {
                monkeys[target_id].items.push(item);
            }
        };
    }

    let most_active: (u64, u64) = monkeys
        .iter()
        .map(|m| m.inspection.performed)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();

    most_active.0 * most_active.1
}
fn create_monkeys(input: &str) -> Vec<Monkey> {
    let cleaned_string: String = input.lines().join("\n"); // should remove carriage returns
    let monkey_strings: Vec<&str> = cleaned_string
        .split("\n\n")
        .filter(|entry| !entry.is_empty())
        .collect();

    let mut monkeys: Vec<Monkey> = vec![];
    for monkey_string in monkey_strings {
        // println!("Monkey String:\n{}\n", monkey_string);
        let mut monkey_iter = monkey_string.lines();
        monkey_iter.next().expect("first line"); // name line, which we ignore

        monkeys.push(Monkey {
            items: monkey_iter
                .next()
                .expect("second line") // items line
                .split_once(": ")
                .expect(": as seperator")
                .1
                .split(", ")
                .map(|item_str| item_str.parse().expect("item is parsable"))
                .collect(),
            inspection: {
                let (operation, value) = monkey_iter
                    .next()
                    .expect("third line")
                    .split_once("old ")
                    .expect("old as seperator")
                    .1
                    .split_once(' ')
                    .expect("space between operation character and number value");
                assert!(operation.chars().count() == 1); // assumption
                Inspection {
                    operation: match operation {
                        "*" => Operation::Multiply,
                        "+" => Operation::Add,
                        _ => panic!("unimplemented operation"),
                    },
                    value: match value {
                        "old" => Value::SelfReferential,
                        _ => Value::Specific(value.parse().expect("operation value is parsable")),
                    },
                    performed: 0,
                }
            },
            test: DivisibleBy(
                monkey_iter
                    .next()
                    .expect("fourth line")
                    .split_once("by ")
                    .expect("by as seperator")
                    .1
                    .parse()
                    .expect("test value is parsable"),
            ),
            outcome: Outcome {
                true_target: {
                    let true_target_str = monkey_iter.next().expect("fifth line");
                    assert!(true_target_str.contains("true:"));
                    true_target_str
                        .split_once("monkey ")
                        .expect("monkey as seperator")
                        .1
                        .parse()
                        .expect("target id is parsable")
                },
                false_target: {
                    let false_target_str = monkey_iter.next().expect("sixth line");
                    assert!(false_target_str.contains("false:"));
                    false_target_str
                        .split_once("monkey ")
                        .expect("monkey as seperator")
                        .1
                        .parse()
                        .expect("target id is parsable")
                },
            },
        });
    }
    monkeys
}
struct Monkey {
    items: Vec<Item>,
    inspection: Inspection,
    test: DivisibleBy,
    outcome: Outcome,
}
type Item = u64; // assume worry levels cannot be zero
struct Inspection {
    operation: Operation,
    value: Value,
    performed: u64,
}
enum Operation {
    Multiply,
    Add,
}
enum Value {
    SelfReferential,
    Specific(u64),
}
struct DivisibleBy(u64);
struct Outcome {
    true_target: MonkeyID,
    false_target: MonkeyID,
}
type MonkeyID = usize;

fn part_2_solve(input: &str) -> u64 {
    let monkeys = &mut create_monkeys(input)[..];
    let divisor_product = monkeys.iter().map(|m| m.test.0).product::<u64>();

    // println!("At the start, the monkeys are holding these worry levels:");
    // for (id, monkey) in monkeys.iter().enumerate() {
    //     println!("Monkey {}: {}", id, monkey.items.iter().join(", "));
    // }
    // println!();

    for _round in 1..=10000 {
        // println!("\n--- ROUND {} ---\n", _round);
        for monkey_id in 0..monkeys.len() {
            let monkey_ref = &monkeys[monkey_id];

            // println!("Monkey {}:", monkey_id);
            let new_items: Vec<(MonkeyID, Item)> = monkey_ref
                .items
                .iter()
                .map(|item| {
                    // println!("  Monkey inspects an item with a worry level of {}", item);
                    let value = match monkey_ref.inspection.value {
                        Value::SelfReferential => *item,
                        Value::Specific(value) => value,
                    };
                    
                    let item = item % divisor_product;
                    
                    let post_inspection = match monkey_ref.inspection.operation {
                        Operation::Multiply => {
                            // print!("    Worry level is multiplied by ");
                            item * value
                        }
                        Operation::Add => {
                            // print!("    Worry level increases by ");
                            item + value
                        }
                    };
                    // println!("{} to {}", value, post_inspection);

                    // let post_worry = post_inspection;
                    // println!("    Monkey gets bored with item. Worry level is divided by 3 to {}", post_worry);
                    let target_id = if post_inspection % monkey_ref.test.0 == 0 {
                        // println!("    Current worry level is divisible by {}.", monkey_ref.test.0);
                        monkey_ref.outcome.true_target
                    } else {
                        // println!("    Current worry level is not divisible by {}.", monkey_ref.test.0);
                        monkey_ref.outcome.false_target
                    };
                    // println!("    Item with worry level {} is (will be) thrown to monkey {}.", post_inspection, target_id);
                    (target_id, post_inspection)
                })
                .collect();

            monkeys[monkey_id].inspection.performed += monkey_ref.items.len() as u64;
            monkeys[monkey_id].items.clear();

            for (target_id, item) in new_items {
                monkeys[target_id].items.push(item);
            }
        };
    }

    let most_active: (u64, u64) = monkeys
        .iter()
        .map(|m| m.inspection.performed)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();

    println!("Most active: {} & {}", most_active.0, most_active.1);
    most_active.0 * most_active.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1_solve(include_str!("sample_1.txt")), 10605);
    }
    #[test]
    fn part_2_test() {
        assert_eq!(part_2_solve(include_str!("sample_1.txt")), 2713310158);
    }
}
