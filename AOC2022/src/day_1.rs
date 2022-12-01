use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
    fn total_food_calories(&self) -> u32 {
        let mut total: u32 = 0;
        for food_item in self.food_items.as_slice() {
            total += food_item.calories;
        }
        total
    }
}

const PATH: &str = "src/day_1.txt";

pub fn run() {
    let path: &Path = Path::new(PATH);
    println!("Trying to read path: {}, absolute: {}, in directory: {}", path.display(), path.is_absolute(), env::current_dir().expect("No directory?").display());
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

    let mut elf_with_most_food = &elves[0];
    for elf in &elves {
        if elf.total_food_calories() >= elf_with_most_food.total_food_calories() {
            elf_with_most_food = &elf;
        }
    }

    println!("Elf with most calories has {}", elf_with_most_food.total_food_calories())
}