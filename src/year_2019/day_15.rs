use std::collections::{HashMap, HashSet, hash_map::Entry};

use num_enum::TryFromPrimitive;
use strum::IntoEnumIterator;

use crate::{
    program::{
        Program, Val,
        io::TryWriteVal,
        spawn::{self, ProgramHandle},
    },
    utls::{
        ValInto,
        color::Color,
        display,
        linalg::{CardinalDir, Point},
    },
};

struct DfsCtx<'a> {
    visited: HashMap<Point, usize>,
    program_handle: &'a mut ProgramHandle,
    oxygen_system: Option<Point>,
}

impl<'a> DfsCtx<'a> {
    pub fn new(program_handle: &'a mut ProgramHandle) -> Self {
        Self {
            visited: Default::default(),
            program_handle,
            oxygen_system: None,
        }
    }
    pub fn dfs(&mut self, depth: usize, point: Point) {
        match self.visited.entry(point) {
            Entry::Occupied(mut occupied_entry) => {
                let best_depth = occupied_entry.get_mut();
                if *best_depth <= depth {
                    return;
                }

                *best_depth = depth;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(depth);
            }
        }

        // // we don't need to continue our recurse if our current point is the oxygen system
        // if self.oxygen_system == Some(point) {
        //     return;
        // }

        // if we got here, then we found a new best path
        for dir in CardinalDir::iter() {
            let new_pos = point + dir.into();
            // step forwawrd
            self.program_handle.try_write_val(dir as Val);
            // check move
            let move_res: DroidState = self.program_handle.try_read_val().unwrap().val_into();
            match move_res {
                DroidState::Wall => {
                    continue;
                }
                DroidState::Open => {}
                DroidState::OxygenSystem => {
                    self.oxygen_system = Some(new_pos);
                }
            }
            // recurse dfs
            self.dfs(depth + 1, point + dir.into());
            // Undo
            self.program_handle.try_write_val(-dir as Val);
            // we know that this will succeed
            self.program_handle.try_read_val();
        }
    }
}

// fn part_1(program: Program) -> usize {
//     let mut program_handle = spawn::spawn(program);
//     let mut dfs_ctx = DfsCtx::new(&mut program_handle);
//     dfs_ctx.dfs(0, Point::default());
//     let oxygen_point = dfs_ctx.oxygen_system.unwrap();
//     dfs_ctx.visited[&oxygen_point]
// }

fn part_2(program: Program) -> usize {
    let mut program_handle = spawn::spawn(program);
    let mut dfs_ctx = DfsCtx::new(&mut program_handle);
    dfs_ctx.dfs(0, Point::default());
    let oxygen_point = dfs_ctx.oxygen_system.unwrap();
    let mut level = vec![oxygen_point];

    let mut visited = HashSet::new();
    let mut real_depth = 0;
    let paintable: HashMap<_, Color> = dfs_ctx
        .visited
        .keys()
        .map(|point| (*point, Color::White))
        .collect();
    display::paint(&paintable);
    while !level.is_empty() {
        let mut new_level = vec![];
        let mut level_effective_nonempty = false;
        for cur in level {
            if !visited.insert(cur) {
                continue;
            }

            level_effective_nonempty = true;

            new_level.extend(
                cur.neighbors()
                    .filter(|n| dfs_ctx.visited.contains_key(n) && !visited.contains(n)),
            );
        }
        level = new_level;
        if level_effective_nonempty {
            real_depth += 1;
        }
    }
    real_depth - 1
}

#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum DroidState {
    Wall = 0,
    Open = 1,
    OxygenSystem = 2,
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "15");
    // let res = part_1(input.clone());
    let res = part_2(input);
    println!("The answer is {}", res);
}
