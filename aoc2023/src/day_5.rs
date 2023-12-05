use itertools::Itertools;
use std::cell::Cell;
use std::collections::hash_map::Values;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::str::FromStr;
use strum::EnumString;

pub(crate) fn part_1(input: &str) -> String {
    struct Data {
        type_name: String,
        num: usize,
    }
    let mut lines = Cell::new(input.lines());
    let (_seed_label, seed_ids) = lines.get_mut().next().unwrap().split_once(": ").unwrap();
    let values = seed_ids
        .split_ascii_whitespace()
        .map(|id_str| Data {
            type_name: String::from("seed"),
            num: id_str.parse().unwrap(),
        })
        .collect::<Vec<_>>();
    let _space = lines.get_mut().next().unwrap();

    #[derive(Debug, PartialEq)]
    struct RangeMapping<T> {
        source: Range<T>,
        dest: Range<T>,
    }
    let mut maps: HashMap<String, (String, Vec<RangeMapping<_>>)> = HashMap::new();
    while let Some(label) = lines.get_mut().next() {
        dbg!(label);
        let (source_type_name, rest) = label.split_once("-to-").unwrap();
        let (dest_type_name, _map_label) = rest.split_once(' ').unwrap();
        let previous_value = maps.insert(source_type_name.to_string(), {
            let mut map = lines
                .get_mut()
                .take_while(|line| !line.is_empty())
                .map(|data_mapping| {
                    let (dest_start, source_start, range_length) = data_mapping
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    let (dest, source) = (
                        dest_start..(dest_start + range_length),
                        source_start..(source_start + range_length),
                    );
                    RangeMapping { source, dest }
                })
                .collect();
            (dest_type_name.to_string(), map)
        });
        assert_eq!(previous_value, None);
    }
    values
        .into_iter()
        .map(|mut data| {
            while let Some((dest_type_name, range_mappings)) = maps.get(&data.type_name) {
                dbg!(dest_type_name);
                data.type_name = dest_type_name.clone();
                data.num = if let Some(mapping) = range_mappings
                    .iter()
                    .find(|range_mapping| range_mapping.source.contains(&data.num))
                {
                    let local_offset = data.num - mapping.source.start;
                    mapping.dest.start + local_offset
                } else {
                    data.num
                }
            }
            data.num
        })
        .min()
        .unwrap()
        .to_string()
}

/// Note:
/// For the real input, this takes time in the magnitude of a minute/minutes to run,
/// even on decently fast hardware with release optimizations.
pub(crate) fn part_2(input: &str) -> String {
    #[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, EnumString)]
    enum DataVariant {
        #[strum(ascii_case_insensitive)]
        Seed,
        #[strum(ascii_case_insensitive)]
        Soil,
        #[strum(ascii_case_insensitive)]
        Fertilizer,
        #[strum(ascii_case_insensitive)]
        Water,
        #[strum(ascii_case_insensitive)]
        Light,
        #[strum(ascii_case_insensitive)]
        Temperature,
        #[strum(ascii_case_insensitive)]
        Humidity,
        #[strum(ascii_case_insensitive)]
        Location,
    }
    #[derive(Clone)]
    struct Data {
        variant: DataVariant,
        num: usize,
    }
    impl Debug for Data {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{:?}: {}]", self.variant, self.num)
        }
    }
    let mut lines = Cell::new(input.lines());
    let (_seed_label, seed_data) = lines.get_mut().next().unwrap().split_once(": ").unwrap();
    let seed_ranges = seed_data
        .split_ascii_whitespace()
        .tuples()
        .map(|(range_start, range_length)| {
            let (range_start, range_length) = [range_start, range_length]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            range_start..(range_start + range_length)
        })
        .collect::<Vec<_>>();
    let _space = lines.get_mut().next().unwrap();
    // dbg!(&seed_ranges);

    #[derive(Debug, PartialEq)]
    struct RangeMapping<T> {
        source: Range<T>,
        dest: Range<T>,
    }
    let mut maps: BTreeMap<DataVariant, (DataVariant, Vec<RangeMapping<_>>)> = BTreeMap::new();
    while let Some(label) = lines.get_mut().next() {
        let (source_type_name, rest) = label.split_once("-to-").unwrap();
        let (dest_type_name, _map_label) = rest.split_once(' ').unwrap();
        let previous_value = maps.insert(source_type_name.parse().unwrap(), {
            let mut map = lines
                .get_mut()
                .take_while(|line| !line.is_empty())
                .map(|data_mapping| {
                    let (dest_start, source_start, range_length) = data_mapping
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    let (dest, source) = (
                        dest_start..(dest_start + range_length),
                        source_start..(source_start + range_length),
                    );
                    RangeMapping { source, dest }
                })
                .collect();
            (dest_type_name.parse().unwrap(), map)
        });
        assert_eq!(previous_value, None);
    }

    /// #[cfg(test)]
    /// {
    ///     {
    ///         const TRACKED_SEED: usize = 82;
    ///         eprintln!(" --- START EXAMPLE TRACKING OF {} --- ", TRACKED_SEED);
    ///         debug_assert!(values.iter().any(|data| data.num == TRACKED_SEED));
    ///         let mut tracked_data = Data {
    ///             type_name: String::from("seed"),
    ///             num: TRACKED_SEED,
    ///         };
    ///         while let Some((dest_type_name, range_mappings)) = maps.get(&tracked_data.type_name) {
    ///             let prev = tracked_data.clone();
    ///             tracked_data.type_name = dest_type_name.clone();
    ///             tracked_data.num = if let Some(mapping) = range_mappings
    ///                 .iter()
    ///                 .find(|range_mapping| range_mapping.source.contains(&tracked_data.num))
    ///             {
    ///                 let local_offset = tracked_data.num - mapping.source.start;
    ///                 mapping.dest.start + local_offset
    ///             } else {
    ///                 tracked_data.num
    ///             };
    ///             eprintln!("Converted {:?} to {:?}", prev, tracked_data);
    ///         }
    ///         eprintln!(" --- ENDED EXAMPLE TRACKING --- ");
    ///     }
    ///
    ///     {
    ///         // find problematic seed in sample
    ///         eprintln!(" --- START PROBLEMATIC SEED HUNT --- ");
    ///         const PROBLEMATIC_RESULT: usize = 1;
    ///         let problematic_seed_indices = values
    ///             .clone()
    ///             .into_iter()
    ///             .enumerate()
    ///             .map(|(index, mut data)| {
    ///                 while let Some((dest_type_name, range_mappings)) = maps.get(&data.type_name) {
    ///                     data.type_name = dest_type_name.clone();
    ///                     data.num = if let Some(mapping) = range_mappings
    ///                         .iter()
    ///                         .find(|range_mapping| range_mapping.source.contains(&data.num))
    ///                     {
    ///                         let local_offset = data.num - mapping.source.start;
    ///                         mapping.dest.start + local_offset
    ///                     } else {
    ///                         data.num
    ///                     };
    ///                 }
    ///                 (index, data)
    ///             })
    ///             .filter(|(_index, data)| data.num == PROBLEMATIC_RESULT)
    ///             .map(|(index, _)| index)
    ///             .collect::<Vec<_>>();
    ///         debug_assert_eq!(problematic_seed_indices.len(), 1);
    ///         let problematic_index = problematic_seed_indices[0];
    ///         let mut problematic_data = values[problematic_index].clone();
    ///         eprintln!(
    ///             "Problematic seed found: {:?}, index {} of values.",
    ///             problematic_data, problematic_index
    ///         );
    ///         while let Some((dest_type_name, range_mappings)) = maps.get(&problematic_data.type_name)
    ///         {
    ///             let prev = problematic_data.clone();
    ///             problematic_data.type_name = dest_type_name.clone();
    ///             problematic_data.num = if let Some(mapping) = range_mappings
    ///                 .iter()
    ///                 .find(|range_mapping| range_mapping.source.contains(&problematic_data.num))
    ///             {
    ///                 let local_offset = problematic_data.num - mapping.source.start;
    ///                 mapping.dest.start + local_offset
    ///             } else {
    ///                 problematic_data.num
    ///             };
    ///             eprintln!("Converted {:?} to {:?}", prev, problematic_data);
    ///         }
    ///         eprintln!(" --- ENDED PROBLEMATIC SEED HUNT --- ");
    ///     }
    /// }
    let mut tried_ranges: Vec<Range<_>> = vec![];
    seed_ranges
        .into_iter()
        .filter_map(|range| {
            eprintln!("Finding min for range {:?}", range);
            let min = range
                .clone()
                .filter(|num| {
                    !tried_ranges
                        .iter()
                        .any(|tried_range| tried_range.contains(num))
                })
                .map(|num| {
                    let mut data = Data {
                        variant: DataVariant::Seed,
                        num,
                    };
                    while let Some((dest_variant, range_mappings)) = maps.get(&data.variant) {
                        data.variant = dest_variant.clone();
                        data.num = if let Some(mapping) = range_mappings
                            .iter()
                            .find(|range_mapping| range_mapping.source.contains(&data.num))
                        {
                            let local_offset = data.num - mapping.source.start;
                            mapping.dest.start + local_offset
                        } else {
                            data.num
                        };
                    }
                    data.num
                })
                .min();
            eprintln!("Min in {:?} (of values not tried): {:?}", range, min);
            tried_ranges.push(range);
            min
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), "35");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), "46");
    }
}