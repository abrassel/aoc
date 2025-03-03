use std::{
    cmp,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};

use bit_set::BitSet;
use itertools::Itertools;
use maplit::{hashmap, hashset};

use crate::{
    maze::{self, Maze},
    utls::linalg::Point,
};

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Edge {
    required: BitSet,
    weight: usize,
}
#[derive(Debug)]
pub struct Graph {
    adj: HashMap<char, HashSet<char>>,
    weights: HashMap<(char, char), Edge>,
    bitmap: HashMap<char, usize>,
}

impl From<&Maze> for Graph {
    fn from(maze: &Maze) -> Self {
        // run bfs from each key and start, recording all adjacent keys
        // do not include any transitive key paths, since they'll already be transitive in *that* key's BFS.
        // this means that our BFS terminates upon reaching another key
        let starts = maze
            .things
            .iter()
            .filter(|(_, c)| {
                c.is_lowercase() || **c == '@' || **c == '&' || **c == '$' || **c == '%'
            })
            .collect_vec();

        let bitmap: HashMap<_, _> = starts
            .iter()
            .enumerate()
            .map(|(idx, (_, char))| (**char, idx))
            .collect();

        struct ToVisit {
            originator: char,
            cur_point: Point,
            edge: Edge,
        }

        impl ToVisit {
            pub fn new(orig: char, cur: Point) -> Self {
                Self {
                    originator: orig,
                    cur_point: cur,
                    edge: Default::default(),
                }
            }
        }
        let mut to_visit = starts
            .into_iter()
            .map(|(point, char)| ToVisit::new(*char, *point))
            .collect_vec();
        let mut visited: HashSet<(Point, BitSet)> = HashSet::new();
        let mut bests = HashMap::new();

        while let Some(ToVisit {
            originator,
            cur_point,
            edge: Edge {
                mut required,
                weight,
            },
        }) = to_visit.pop()
        {
            let tuple = (cur_point, required.clone());
            if visited.contains(&tuple) {
                continue;
            }

            visited.insert(tuple);

            match maze.things.get(&cur_point) {
                Some(x) if x.is_ascii_lowercase() => {
                    // found a key
                    // bfsing, so this is always going to be the best score
                    bests.insert(
                        (originator, *x),
                        Edge {
                            required: required.clone(),
                            weight,
                        },
                    );
                    // will not continue after finding another key, since we do not do transitive paths
                    continue;
                }
                Some(x) if x.is_ascii_uppercase() => {
                    // found a door
                    // add its key to the requirements list
                    required.insert(bitmap[&x.to_ascii_lowercase()]);
                }
                _ => {
                    // do nothing, since we're just a normal, empty spot
                    // we are trying to eliminate these spots from the graph
                }
            }

            // bfs through neighbors
            let new_edge = Edge {
                weight: weight + 1,
                required,
            };
            for neighbor in maze.neighbors(&cur_point) {
                to_visit.push(ToVisit {
                    originator,
                    cur_point: neighbor,
                    edge: new_edge.clone(),
                });
            }
        }

        let mut adj: HashMap<char, HashSet<char>> = HashMap::new();
        for (from, to) in bests.keys() {
            adj.entry(*from).or_default().insert(*to);
        }

        Self {
            adj,
            weights: bests,
            bitmap,
        }
    }
}

impl Graph {
    pub fn astar(&self, start_pos: impl Iterator<Item = char>) -> usize {
        #[derive(Eq, PartialEq)]
        struct State {
            cost: Edge,
            state: Vec<char>,
        }
        impl std::cmp::Ord for State {
            fn cmp(&self, other: &Self) -> cmp::Ordering {
                other
                    .cost
                    .weight
                    .cmp(&self.cost.weight)
                    .then_with(|| self.cost.required.len().cmp(&other.cost.required.len()))
                    .then_with(|| self.state.cmp(&other.state))
            }
        }
        impl std::cmp::PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        let mut to_visit = BinaryHeap::new();
        let start_state = start_pos.collect_vec();
        let edge = Edge {
            // bootstrap visited with the start positions.
            required: start_state.iter().map(|x| self.bitmap[x]).collect(),
            weight: 0,
        };
        to_visit.push(State {
            cost: edge,
            state: start_state,
        });
        let mut visited = HashSet::new();
        let desired: BitSet = self.bitmap.values().copied().collect();
        while let Some(state) = to_visit.pop() {
            let visited_state = (state.cost.required, state.state);
            let weight = state.cost.weight;

            if visited.contains(&visited_state) {
                continue;
            }

            visited.insert(visited_state.clone());
            let keys = visited_state.0;
            if keys == desired {
                return state.cost.weight;
            }

            // try each neighbor
            let state = visited_state.1;
            for i in 0..state.len() {
                for nbor in &self.adj[&state[i]] {
                    let edge = &self.weights[&(state[i], *nbor)];
                    if keys.is_superset(&edge.required) {
                        let new_state = State {
                            cost: Edge {
                                required: {
                                    let mut keys = keys.clone();
                                    keys.insert(self.bitmap[nbor]);
                                    keys
                                },
                                weight: weight + edge.weight,
                            },
                            state: {
                                let mut state = state.clone();
                                state[i] = *nbor;
                                state
                            },
                        };
                        to_visit.push(new_state);
                    }
                }
            }
        }
        unreachable!("There was no solution");
    }
}

fn part_1(maze: &Maze) -> usize {
    let starts = maze
        .things
        .iter()
        .filter(|(_point, val)| **val == '@' || **val == '&' || **val == '$' || **val == '%')
        .map(|(_, chars)| *chars);

    let graph = Graph::from(maze);
    graph.astar(starts)
}

fn part_2(maze: &Maze) -> usize {
    part_1(maze)
}

pub fn run() {
    let input: Maze = crate::utls::read_text_from_file("2019", "18");
    // let res = part_1(&input);
    // println!("The answer is: {}", res);
    let res = part_2(&input);
    println!("The answer is: {}", res);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_solution_0() {
        let input = "#########
#b.A.@.a#
#########";

        let maze = Maze::my_parse(input);
        let res = part_1(&maze);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_solution_1() {
        let input = "#########
#b.A.@.a#
#########";

        let maze = Maze::my_parse(input);
        let res = part_1(&maze);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_solution_2() {
        let input = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#$BcIJ#
#############
#nK.L%#&G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

        let maze = Maze::my_parse(input);
        let graph = Graph::from(&maze);
        println!("{:?}", graph);
    }
}
