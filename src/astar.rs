use crate::constants::{GRID_COLS, GRID_ROWS};
use crate::utils::{Block, Map, Vector};
use std::collections::HashSet;

// TODO: Switch to using manhattan_distance funtion from utils.rs
fn calc_herustic(position: Vector, destination: Vector) -> i32 {
    let dx = (position.x - destination.x).abs();
    let dy = (position.y - destination.y).abs();
    let dx_wrap = (GRID_COLS as i32 - dx).abs();
    let dy_wrap = (GRID_ROWS as i32 - dy).abs();
    let min_dx = dx.min(dx_wrap);
    let min_dy = dy.min(dy_wrap);
    min_dx + min_dy
}

fn get_neighbours(map: &Map, position: Vector) -> Vec<Block> {
    let mut neigh: Vec<Block> = Vec::new();
    let offsets = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for offset in offsets {
        let x = ((position.x + offset.0) + GRID_COLS as i32) % GRID_COLS as i32;
        let y = ((position.y + offset.1) + GRID_ROWS as i32) % GRID_ROWS as i32;

        if !map[y as usize][x as usize].is_wall {
            neigh.push(map[y as usize][x as usize]);
        }
    }

    return neigh;
}

fn neighbour_with_min_f(open_set: &HashSet<Block>) -> Block {
    let mut result = open_set.iter().nth(0).unwrap();
    for block in open_set.iter() {
        if block.f_val < result.f_val {
            result = block;
        }
    }
    return *result;
}

fn has_greater_g_val(open_set: &HashSet<Block>, neigh: &Block) -> bool {
    for block in open_set.iter() {
        if neigh.g_val > block.g_val {
            return true;
        }
    }
    false
}

pub fn find_path(map: &mut Map, start: Vector, destination: Vector) -> Vec<Vector> {
    let mut open_set = HashSet::<Block>::new();
    let mut closed_set = HashSet::<Block>::new();

    open_set.insert(map[start.y as usize][start.x as usize]);

    while !open_set.is_empty() {
        let current = neighbour_with_min_f(&open_set);

        closed_set.insert(current);
        open_set.remove(&current);

        if current.pos == destination {
            break;
        }

        let neighbours = get_neighbours(&map, current.pos);
        for mut neigh in neighbours {
            if closed_set.contains(&neigh) {
                continue;
            }

            // Calculate G and F for neigh
            neigh.g_val = current.g_val + 1;
            neigh.f_val = neigh.g_val + calc_herustic(neigh.pos, destination) as u16;

            // Set current as parent for neigh
            neigh.parent_pos = current.pos;

            // Push current and neigh to map
            map[current.pos.y as usize][current.pos.x as usize] = current;
            map[neigh.pos.y as usize][neigh.pos.x as usize] = neigh;

            if open_set.contains(&neigh) && has_greater_g_val(&open_set, &neigh) {
                continue;
            }

            open_set.insert(neigh);
        }
    }

    let mut path = Vec::<Vector>::new();
    let mut current = map[destination.y as usize][destination.x as usize];
    while current.pos.x != start.x || current.pos.y != start.y {
        path.push(current.pos);
        current = map[current.parent_pos.y as usize][current.parent_pos.x as usize];
    }

    return path;
}
