use crate::Runnable;

mod grid;
use grid::*;
use itertools::Itertools;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let input = input.as_str();
        println!("Visible trees outside grid: {}", part_1_solve(input));
        println!("Score of the place with the highest scenic score: {}", part_2_solve(input));
    }
}

// how many trees are visible outside the grid?
fn part_1_solve(input: &str) -> usize {
    let grid = parse_grid(input);

    let all_rows = (0..grid.height()).flat_map(|y| {
        let left: Box<dyn Iterator<Item = GridCoord>> = Box::new(
            (0..grid.width())
                .map(move |x| GridCoord::from((x, y)))
        );
        let right = Box::new(
            (0..grid.width())
                .rev()
                .map(move |x| GridCoord::from((x, y)))
        );
        [left as Box<dyn Iterator<Item = GridCoord>>, right].into_iter()
    });

    let all_columns = (0..grid.width()).flat_map(|x| {
        let top: Box<dyn Iterator<Item = GridCoord>> = Box::new(
            (0..grid.height())
                .map(move |y| GridCoord::from((x, y)))
        );
        let bottom = Box::new(
            (0..grid.height())
                .rev()
                .map(move |y| GridCoord::from((x, y)))
        );
        [top as Box<dyn Iterator<Item = GridCoord>>, bottom].into_iter()
    });

    let all_lines = all_rows.chain(all_columns);
    let all_visible = all_lines
        .flat_map(|it| {
            let mut it = it
                .map(|coord| (coord, grid.cell(coord).unwrap()))
                .peekable();
            let first = it.peek().unwrap().0;
            std::iter::once(first).chain(
                it.coalesce(|(a_coord, a_tree), (b_coord, b_tree)| {
                    if b_tree.height <= a_tree.height {
                        Ok((a_coord, a_tree))
                    } else {
                        Err(((a_coord, a_tree), (b_coord, b_tree)))
                    }
                })
                .tuple_windows().map_while(
                    |((_previous_coord, previous_tree), (current_coord, current_tree))| {
                        if previous_tree.height <= current_tree.height {
                            Some(current_coord)
                        } else {
                            None
                        }
                    }
                )
            )
        });
    
    all_visible.unique().collect_vec().len()
}

#[derive(Default, Clone)]
#[repr(transparent)]
struct Tree { height: u8 }

fn parse_grid(input: &str) -> Grid<Tree> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    
    let mut grid: Grid<Tree> = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, column) in line.trim().chars().enumerate() {
            assert!(column.is_ascii_digit());
            let tree = grid.cell_mut((x, y).into()).unwrap();
            *tree = Tree { height: { (column as usize - '0' as usize) as u8 }};
        }
    }
    grid
}

fn part_2_solve(input: &str) -> usize {
    let grid = parse_grid(input);

    let all_coords = (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    let best_place = all_coords
        .map(|coord| (coord, scenic_score(&grid, coord)))
        .max_by_key(|(_, score)| *score)
        .unwrap();
    
    best_place.1 // highest score
}

fn visible_trees_in_dir(grid: &Grid<Tree>, coord: GridCoord, (dx, dy): (isize, isize)) -> usize {
    let line = (1..).map_while(|i| {
        let coord = GridCoord {
            x: coord.x.checked_add_signed(dx * i)?,
            y: coord.y.checked_add_signed(dy * i)?,
        };
        grid.cell(coord)
    });

    let mut total = 0;
    let our_tree = grid.cell(coord).unwrap();
    for tree in line {
        total += 1;
        if tree.height >= our_tree.height {
            break;
        }
    }
    total
}

fn scenic_score(grid: &Grid<Tree>, coord: GridCoord) -> usize {
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    dirs.into_iter()
        .map(|(dx, dy)| visible_trees_in_dir(grid, coord, (dx, dy)))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "30373
        25512
        65332
        33549
        35390";

    mod part_1 {
        use super::*;
        #[test]
        fn test_solver() {
            let test_result = part_1_solve(TEST_INPUT);
            assert_eq!(test_result, 21_usize);
        }
    }
    mod part_2 {
        use super::*;
        #[test]
        fn test_solver() {
            let test_result = part_2_solve(TEST_INPUT);
            assert_eq!(test_result, 8_usize);
        } 
    }
}