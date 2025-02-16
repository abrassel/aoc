use std::{collections::HashMap, convert::Infallible, str::FromStr};

use itertools::Itertools;

fn part_1(relations: &HashMap<String, Relation>) -> usize {
    fuel_to_ore(relations, 1)
}

fn fuel_to_ore(relations: &HashMap<String, Relation>, fuel_count: usize) -> usize {
    let mut materials: HashMap<_, usize> = HashMap::new();
    let mut make_counts = HashMap::new();
    let mut to_make = Vec::new();

    make_counts.insert("FUEL", fuel_count);
    to_make.push("FUEL");

    while let Some(reagent) = to_make.pop() {
        if let Some(Relation { inputs, factor, .. }) = relations.get(reagent) {
            let mut requested_count: usize = make_counts.remove(reagent).unwrap();
            if let Some(available) = materials.get_mut(reagent) {
                // transfer materials over
                let new_requested_count = requested_count.saturating_sub(*available);
                *available = available.saturating_sub(requested_count);
                requested_count = new_requested_count;
            }

            if requested_count > 0 {
                // relation will manufactor `factor` at a time, so manufactor possible surplus
                let multiple = requested_count.div_ceil(*factor);

                // compute the surplus here and add it to our available supply
                let surplus = factor * multiple - requested_count;
                if surplus > 0 {
                    *materials.entry(reagent).or_default() += surplus;
                }

                // add all of our upstream dependencies
                for (amt, input) in inputs {
                    let amt = amt * multiple;
                    match make_counts.entry(input) {
                        std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                            // there was already a request for this resource, so imply request more
                            *occupied_entry.get_mut() += amt;
                        }
                        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                            // we have not yet requested this resource, so enqueue it and insert for the first time
                            to_make.push(input);
                            vacant_entry.insert(amt);
                        }
                    }
                }
            }
            // only "ORE" does not have a reagent - but we are trying to accumulate it.
        }
    }

    make_counts["ORE"]
}

fn part_2(relations: &HashMap<String, Relation>) -> usize {
    let mut low = 1000000000000usize.div_ceil(873899);
    let mut high = low * 2;

    while low < high {
        let mid = (low + high) / 2;
        let ore_used = fuel_to_ore(relations, mid);
        match ore_used.cmp(&1000000000000) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal => {
                return mid;
            }
            std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }

    println!(
        "Sanity check: {} gives us {}, and {} gives us {}\nSanity check: {} gives us {}, and {} gives us {}",
        low,
        fuel_to_ore(relations, low),
        low - 1,
        fuel_to_ore(relations, low - 1),
        low,
        1000000000000usize,
        low - 1,
        1000000000000usize
    );

    low - 1
}

#[derive(Clone)]
struct Relation {
    inputs: Vec<(usize, String)>,
    name: String,
    factor: usize,
}

impl FromStr for Relation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        let (input, output) = s.trim().split_once(" => ").unwrap();
        let (count, output) = output.trim().split_once(" ").unwrap();
        let inputs = input
            .split(", ")
            .map(|input| {
                let (count, input) = input.trim().split_once(" ").unwrap();
                (count.trim().parse().unwrap(), input.trim().to_string())
            })
            .collect_vec();

        Ok(Self {
            inputs,
            name: output.trim().to_string(),
            factor: count.trim().parse().unwrap(),
        })
    }
}

pub fn run() {
    let input: Vec<Relation> = crate::utls::read_text_from_file("2019", "14");
    let input = input
        .into_iter()
        .map(|relation| (relation.name.clone(), relation))
        .collect();
    let res = part_1(&input);
    println!("The answer is {}", res);
    let res = part_2(&input);
    println!("The answer is {}", res);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_solution_0() {
        let input = "
            10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let input: Vec<Relation> = Vec::my_parse(input);
        let input = input
            .into_iter()
            .map(|relation| (relation.name.clone(), relation))
            .collect();
        let res = part_1(&input);
        assert_eq!(res, 31);
    }

    #[test]
    fn test_solution_1() {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let input: Vec<Relation> = Vec::my_parse(input);
        let input = input
            .into_iter()
            .map(|relation| (relation.name.clone(), relation))
            .collect();
        let res = part_1(&input);
        assert_eq!(res, 165);
    }

    #[test]
    fn test_solution_2() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let input: Vec<Relation> = Vec::my_parse(input);
        let input = input
            .into_iter()
            .map(|relation| (relation.name.clone(), relation))
            .collect();
        let res = part_1(&input);
        assert_eq!(res, 13312);
    }

    #[test]
    fn test_solution_3() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let input: Vec<Relation> = Vec::my_parse(input);
        let input = input
            .into_iter()
            .map(|relation| (relation.name.clone(), relation))
            .collect();
        let res = part_1(&input);
        assert_eq!(res, 180697);
    }

    #[test]
    fn test_solution_4() {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let input: Vec<Relation> = Vec::my_parse(input);
        let input = input
            .into_iter()
            .map(|relation| (relation.name.clone(), relation))
            .collect();
        let res = part_1(&input);
        assert_eq!(res, 2210736);
    }

    #[test]
    fn test_solution_5() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let input: Vec<Relation> = Vec::my_parse(input);
        let input = input
            .into_iter()
            .map(|relation| (relation.name.clone(), relation))
            .collect();
        let res = part_2(&input);
        assert_eq!(res, 82892753);
    }
}
