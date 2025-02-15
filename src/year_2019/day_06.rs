use std::{collections::HashMap, convert::Infallible, str::FromStr};

use itertools::Itertools;

// fn part_1(graph: Vec<Orbit>) -> usize {
//     let dag = Dag::from_iter(graph);
//     dag.count_orbits()
// }

fn part_2(graph: Vec<Orbit>) -> usize {
    let dag = Dag::from_iter(graph);
    let path = dag.paths_from_root();
    path.shortest_path("YOU", "SAN") - 2
}

struct Orbit {
    from: String,
    to: String,
}

struct Dag {
    map: HashMap<String, Vec<String>>,
}

struct Path<'a>(HashMap<&'a str, &'a str>);

impl<'a> Path<'a> {
    pub fn walk_rev(&'a self, end: &'a str) -> Vec<&'a str> {
        let mut cur = end;
        let mut path = vec![cur];
        while let Some(&prev) = self.0.get(cur) {
            path.push(prev);
            cur = prev;
        }
        path
    }

    pub fn shortest_path(&self, a: &str, b: &str) -> usize {
        let a_walk = self.walk_rev(a);
        let b_walk = self.walk_rev(b);
        let a_map: HashMap<&str, usize> = a_walk
            .iter()
            .enumerate()
            .map(|(idx, &val)| (val, idx))
            .collect();

        // find first (last, because reverse) index in b also in a
        let (bdist, merge_base) = b_walk
            .into_iter()
            .find_position(|b| a_map.contains_key(b))
            .unwrap();
        let adist = a_map[merge_base];
        // we know how far away from b and a the merge base is, so the sum is the distance from the merge base to each point
        bdist + adist
    }
}

impl Dag {
    pub fn neighbors(&self, val: &str) -> &[String] {
        match self.map.get(val) {
            Some(val) => val,
            None => &[],
        }
    }

    pub fn root(&self) -> &str {
        "COM"
    }

    // pub fn count_orbits(&self) -> usize {
    //     let mut frontier = vec![self.root()];
    //     let mut total_orbits = 0;
    //     let mut level = 0;
    //     while !frontier.is_empty() {
    //         let mut new_frontier = vec![];
    //         for cur in frontier {
    //             total_orbits += level;
    //             for nbor in self.neighbors(cur) {
    //                 new_frontier.push(nbor.as_str());
    //             }
    //         }
    //         frontier = new_frontier;
    //         level += 1;
    //     }

    //     total_orbits
    // }

    pub fn paths_from_root(&self) -> Path {
        let mut frontier = vec![(None, self.root())];
        let mut paths = HashMap::new();
        while !frontier.is_empty() {
            let mut new_frontier = vec![];
            for (prev, cur) in frontier {
                if let Some(prev) = prev {
                    paths.insert(cur, prev);
                }
                for nbor in self.neighbors(cur) {
                    new_frontier.push((Some(cur), nbor.as_str()));
                }
            }
            frontier = new_frontier;
        }

        Path(paths)
    }
}

impl FromIterator<Orbit> for Dag {
    fn from_iter<T: IntoIterator<Item = Orbit>>(iter: T) -> Self {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for Orbit { from, to } in iter {
            map.entry(from).or_default().push(to);
        }

        Self { map }
    }
}

impl FromStr for Orbit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.trim().split_once(")").unwrap();
        Ok(Self {
            from: from.to_owned(),
            to: to.to_owned(),
        })
    }
}

pub fn run() {
    let input = crate::utls::read_text_from_file("2019", "06");
    let res = part_2(input);
    println!("The result is: {}", res);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_orbits() {
        let input = r"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let orbits = Vec::<Orbit>::my_parse(input);
        let res = part_2(orbits);
        assert_eq!(res, 4);
    }
}
