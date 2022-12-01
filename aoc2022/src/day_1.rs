use std::fs::File;
use std::io::Read;

struct FoodItem {
    calories: u32
}
impl FoodItem {
    fn from(calories: u32) -> Self {
        FoodItem {
            calories
        }
    }
}

struct Elf {
    food_items: Vec<FoodItem>
}
impl Elf {
    fn from(food_items: Vec<FoodItem>) -> Self {
        Elf {
            food_items
        }
    }
    fn total_calories(&self) -> u32 {
        let mut total: u32 = 0;
        for food_item in self.food_items.as_slice() {
            total += food_item.calories;
        }
        total
    }
}

const PATH: &str = "inputs/day_1.txt";

pub fn run() {
    let mut file = File::open(PATH).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let lines = file_content.lines();

    let mut elves: Vec<Elf> = vec![];
    let mut current_food_collection: Vec<FoodItem> = vec![];
    for line in lines {
        if line.is_empty() {
            elves.push(Elf::from(current_food_collection));
            current_food_collection = vec![];
        }
        else {
            let calories:u32 = line.parse().expect("Line could not be parsed into int?");
            current_food_collection.push(FoodItem::from(calories));
        }
    }

    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
    let sorted_elves = elves;

    println!("Elf with the most calories has {} calories.", sorted_elves[0].total_calories());
    println!("Elf with second most calories has {} calories.", sorted_elves[1].total_calories());
    println!("Elf with third most calories has {} calories.", sorted_elves[2].total_calories());

    let top_calories: Vec<u32> = sorted_elves[..=2].into_iter().map(|e| e.total_calories()).collect();
    let total: u32 = top_calories.iter().sum();
    println!("Calories of top three elves: {}", total)
}