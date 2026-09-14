use core::panic;
use std::sync::atomic::AtomicUsize;

use crate::CORRIDOR_WIDTH;

use crate::types::*;

pub static INDEX: AtomicUsize = AtomicUsize::new(0);

fn edge_rooms(r1: &Room, r2: &Room, axis: Axis) -> (usize, Point3, Point3, Axis) {
    match axis {
        Axis::X => {
            let left_mid = Point3(r1.2.0,
                r1.2.1 - ((r1.2.1 - r1.1.1) / 2), 
                r1.2.2 - ((r1.2.2 - r1.1.2) / 2));
            let right_mid = Point3(r2.1.0,
                r2.1.1 + ((r2.2.1 - r2.1.1) / 2), 
                r2.1.2 + ((r2.2.2 - r2.1.2) / 2));
            
            return (r1.0, left_mid, right_mid, axis)
        },
        Axis::Y => {
            let left_mid = Point3(r1.2.0 - ((r1.2.0 - r1.1.0) / 2),
                r1.2.1,
                r1.2.2 - ((r1.2.2 - r1.1.2) / 2));
            let right_mid = Point3(r2.1.0 + ((r2.2.0 - r2.1.0) / 2),
                r2.1.1,
                r2.1.2 + ((r2.2.2 - r2.1.2) / 2));

            return (r1.0, left_mid, right_mid, axis)
        }
        Axis::Z => {
            let left_mid = Point3(r1.2.0  - ((r1.2.0 - r1.1.0) / 2),
                r1.2.1 - ((r1.2.1 - r1.1.1) / 2), 
                r1.2.2);
            let right_mid = Point3(r2.1.0 + ((r2.2.0 - r2.1.0) / 2),
                r2.1.1 + ((r2.2.1 - r2.1.1) / 2), 
                r2.1.2);

            return (r1.0, left_mid, right_mid, axis)
        } 
    }
}

fn generate_edges(rooms: (&[&Room], &[&Room]), axis: Axis, split_pos: Point3, edges: &mut Vec<(usize, Point3, Point3, Axis)>) -> () {
    fn get_point_dist(r: &&Room, p: Point3) -> (i64, i64) {
        return (((p.0 - r.1.0).pow(2) + (p.1 - r.1.1).pow(2) + (p.2 - r.1.2).pow(2)), ((p.0 - r.2.0).pow(2) + (p.1 - r.2.1).pow(2) + (p.2 - r.2.2).pow(2)))
    }

    if rooms.0.is_empty() || rooms.1.is_empty() {
        return
    }

    let mut left_closest = (rooms.0[0], i64::MAX);
    rooms.0.iter().for_each(|r| {
        let d = get_point_dist(r, split_pos);
        if d.0 < left_closest.1 || d.1 < left_closest.1 {
            left_closest = (r, d.0.min(d.1));           
        }
    });

    let mut right_closest = (rooms.1[0], i64::MAX);
    rooms.1.iter().for_each(|r| {
        let d = get_point_dist(r, split_pos);
        if d.0 < right_closest.1 || d.1 < right_closest.1 {
            right_closest = (r, d.0.min(d.1));           
        }
    });

    edges.push(edge_rooms(left_closest.0, right_closest.0, axis));  

}

pub fn orthogonal_paths(edges: Vec<(usize, Point3, Point3, Axis)>, map: Vec<Vec<(usize, Point3, Point3)>>) -> Vec<(usize, Point3, Point3, Axis)> {
    let mut new_edges = Vec::new();

    for e in edges.iter() {
        let start_pos = e.1;
        let target = e.2;
        //let mut delta: i64 = 0;

        //Select starting tile by scanning tile map for a tile that matches the index of the edge
        let starting_tile = map[0].iter().filter(|t| t.0 == e.0).next().unwrap();
        let mut prev_seg = (e.0, e.1, e.1, e.3);
        
        println!("Starting tile: {:#?}", starting_tile);
        let mut current_tile = starting_tile;

        //Make sure that the starting position is correct for the math
        for axis in 0..3 {
            if target[axis] > starting_tile.2[axis] {
                prev_seg.1 = prev_seg.2;
                prev_seg.2[axis] = current_tile.2[axis];
            } else if target[axis] < starting_tile.1[axis] {
                prev_seg.1 = prev_seg.2;
                prev_seg.2[axis] = current_tile.1[axis];
            } else {
                //This coordinate doesn't need to be snapped at all!
                prev_seg.1 = prev_seg.2;
                prev_seg.2[axis] = start_pos[axis];
            }
            new_edges.push(prev_seg)
        
        }

        while prev_seg.2 != target {
            for axis in 0..3 {
                if start_pos[axis] < target[axis] {
                    if prev_seg.2[axis] < target[axis] {
                        if current_tile.2[axis] > target[axis] {
                            //Target is within this tile, so snap to it
                            prev_seg.1 = prev_seg.2;
                            prev_seg.2[axis] = target[axis];

                            //Push
                            new_edges.push(prev_seg);
                        } else {
                            let tile = map[0].iter()
                                    .filter(|t| t.1[axis] == current_tile.2[axis] && t.0 != current_tile.0)
                                    //It may be worth changing this min_by_key to be the closest to target rather than closest to current room
                                    .min_by_key(|t| target[axis] - t.2[axis]);
                            let tile = match tile {
                                    Some(v) => v,
                                    None => { 
                                        print!(
                                            "No room found with bounds that match queried bounds! Critical error in pathing function! Edge: x: {} y: {} z: {} | Axis: {:#?} | Split Point: {}",
                                            current_tile.2.0, current_tile.2.1, current_tile.2.2, e.3, current_tile.2[axis]
                                        );
                                        panic!();
                                    }
                                };

                            //Change prev_seg so that it stretches the correct span
                            prev_seg.1 = prev_seg.2;
                            prev_seg.2[axis] = current_tile.2[axis];

                            new_edges.push(prev_seg);
                            
                            //Move bounds
                            current_tile = tile;
                        } 
                    }
                } else {
                    if prev_seg.2[axis] > target[axis] {
                        if current_tile.1[axis] < target[axis] {
                            prev_seg.1 = prev_seg.2;
                            prev_seg.2[axis] = target[axis];
                            new_edges.push(prev_seg);
                        } else {
                            let tile = map[0].iter()
                                    .filter(|t| (t.1[axis] == current_tile.1[axis] || t.2[axis] == current_tile.1[axis]) && t.0 != current_tile.0)
                                    .min_by_key(|t| t.1[axis] - target[axis]);
                            let tile = match tile {
                                    Some(v) => v,
                                    None => { 
                                        print!(
                                            "No room found with bounds that match queried bounds! Critical error in pathing function! | Edge: x: {} y: {} z: {} | Axis: {:#?} | Split Point: {}",
                                            current_tile.1.0, current_tile.1.1, current_tile.1.2, e.3, current_tile.1[axis]
                                        );
                                        panic!();
                                    }
                                };

                            //Change prev_seg so that it stretches the correct span
                            prev_seg.1 = prev_seg.2;
                            prev_seg.2[axis] = current_tile.1[axis];

                            new_edges.push(prev_seg);

                            //Update current tile
                            current_tile = tile;
                        }
                    }
                }
            }
        }
    }
    return new_edges
}

pub fn create_corridors(edges: Vec<(usize, Point3, Point3, Axis)>) -> Vec<(Point3, Point3)> {
    let mut boxes = Vec::<(Point3, Point3)>::new();
    for e in edges {
        let c1 = Point3(e.1.0 - CORRIDOR_WIDTH, e.1.1 - CORRIDOR_WIDTH, e.1.2 - CORRIDOR_WIDTH);
        let c2 = Point3(e.2.0 + CORRIDOR_WIDTH, e.2.1 + CORRIDOR_WIDTH, e.2.2 + CORRIDOR_WIDTH);
        boxes.push((c1, c2)); 
    }
    boxes
}

pub fn edge_dfs<'a>(root: &'a BSPNode<Tile>, divisions: u32, edges: &mut Vec<(usize, Point3, Point3, Axis)>) -> Vec<&'a Room> {
    if divisions - root.value.split_count >= 1 {
        let mut left_rooms = edge_dfs(root.left.as_deref().unwrap(), divisions, edges);
        let right_rooms = edge_dfs(root.right.as_deref().unwrap(), divisions, edges);

        generate_edges(
            (&left_rooms, &right_rooms),
            root.split_d,
            root.left.as_deref().unwrap().value.rc,
            edges,
        );

        left_rooms.extend(right_rooms);
        left_rooms
    } else {
        let mut rooms = Vec::new();
        if let Some(room) = &root.value.room {
            rooms.push(room);
        }
        return rooms
    }
}