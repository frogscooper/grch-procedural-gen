use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicUsize;
use rand::{self, RngExt};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::fs::File;
use std::io::Write;

pub mod types;
use crate::types::*;
pub mod edges;
use crate::edges::*;
pub mod obj;
use crate::obj::*;

pub const SEED: u64 = 184138956713986453;
pub static INDEX: AtomicUsize = AtomicUsize::new(0);

pub const PLACE_DIST: i64 = 20;
pub const CORRIDOR_OFFSET: Point3 = Point3(10, 4, 2);
pub const CORRIDOR_WIDTH: i64 = 20;

pub const ABS_DIST_X: i64 = 0;
pub const ABS_DIST_Y: i64 = 0;
pub const ABS_DIST_Z: i64 = 0;
pub const ROOM_RANDOM_VARIABILITY: f64 = 0.05;
pub const ROOM_SCALE_FACTOR: f64 = 0.08;

//Chance of a room being in any given tile
pub const ROOM_CHANCE: f64 = 2.0 / 3.0;

pub const SIZE: Point3 = Point3(2048, 2048, 2048);


fn split_dfs(root: &mut BSPNode<Tile>, depth: u32, rng: &mut StdRng) {
    if root.value.split_count < depth {
        root.split(rng);
        split_dfs(root.right.as_mut().unwrap(), depth, rng);
        split_dfs(root.left.as_mut().unwrap(), depth, rng);
    } else {
        return
    }
}

fn construct_room(tile: Tile, rng: &mut StdRng, obj_data: &mut String, v: usize) -> Option<Room> {
    if tile.traversible == false {
        return None
    }

    let dist_from_x: i64 = (tile.get_width() as f64 * rng.random_range(0.0..ROOM_RANDOM_VARIABILITY)) as i64;
    let dist_from_y: i64 = (tile.get_height() as f64 * rng.random_range(0.0..ROOM_RANDOM_VARIABILITY)) as i64;
    let dist_from_z: i64 = (tile.get_depth() as f64 * rng.random_range(0.0..ROOM_RANDOM_VARIABILITY)) as i64;

    let room = Room(
        tile.index,
        Point3(
            ((tile.lc.0 as f64 + tile.get_width() as f64 * ROOM_SCALE_FACTOR) as i64 + (ABS_DIST_X / 2)) + dist_from_x,
            ((tile.lc.1 as f64 + tile.get_height() as f64 * ROOM_SCALE_FACTOR)) as i64 + (ABS_DIST_Y / 2) + dist_from_y,
            ((tile.lc.2 as f64 + tile.get_depth() as f64 * ROOM_SCALE_FACTOR)) as i64 + (ABS_DIST_Z / 2) + dist_from_z
        ),
        Point3(
            ((tile.rc.0 as f64 - tile.get_width() as f64 * ROOM_SCALE_FACTOR) as i64 - (ABS_DIST_X / 2)) - dist_from_x,
            ((tile.rc.1 as f64 - tile.get_height() as f64 * ROOM_SCALE_FACTOR)) as i64 - (ABS_DIST_Y / 2) - dist_from_y,
            ((tile.rc.2 as f64 - tile.get_depth() as f64 * ROOM_SCALE_FACTOR)) as i64 - (ABS_DIST_Z / 2) - dist_from_z
        ),
        true );

    add_obj_box(room.1, room.2, obj_data, v);
    return Some(room);

}

fn build_dfs(root: &mut BSPNode<Tile>, tvec: &mut Vec<Tile>, rng: &mut StdRng, obj_data: &mut String, v: usize) -> () {
    if root.right == None {
            let r= construct_room( root.value, rng, obj_data, v);

            //If a room was constructed, insert it into the tile in the tree node
            if r.is_some() {
                root.value.room = r;
            }
            
            //Push tile to tvec so it can be mapped
            tvec.push(root.value);
    } else {
        build_dfs(root.right.as_deref_mut().unwrap(), tvec, rng, obj_data, v);
        build_dfs(root.left.as_deref_mut().unwrap(), tvec, rng, obj_data, v);       
    }
}

pub fn initbt(size: Point3, divisions: u32) -> () {
    let mut rng = StdRng::seed_from_u64(SEED);

    let mut root = BSPNode{
        value: Tile{index: INDEX.fetch_add(1, Relaxed), lc: Point3(0, 0, 0), rc: size, traversible: false, split_count: 0, room: None},
        right: None,
        left: None,
        split_d: Axis::random_variant()
    };

    let mut obj_data = create_obj();
    let mut v: usize = 0;
    let mut tvec = Vec::<Tile>::new();

    split_dfs(&mut root, divisions, &mut rng);
    build_dfs(&mut root, &mut tvec, &mut rng, &mut obj_data, v);
    eprintln!("Structure construction finished, now drawing map");
    
    //Initialize map
    let mut map = Vec::<Vec<(usize, Point3, Point3)>>::new();

    //Map of vectors of the form:
    // (index: u64, left_corner: Point3, right_corner: point3) 
    tvec.sort_by(|t1, t2| t1.rc.0.cmp(&t2.rc.0));
    map.push(tvec.iter().map(|t| (t.index, t.lc, t.rc)).collect());
    tvec.sort_by(|t1, t2| t1.rc.1.cmp(&t2.rc.1));
    map.push(tvec.iter().map(|t| (t.index, t.lc, t.rc)).collect());
    tvec.sort_by(|t1, t2| t1.rc.2.cmp(&t2.rc.2));
    map.push(tvec.iter().map(|t| (t.index, t.lc, t.rc)).collect());


    //Initialize vector used to store the edges generated in the edge_dfs call chain
    let mut edges = Vec::<(usize, Point3, Point3, Axis)>::new();
  
    edge_dfs(&root, divisions, &mut edges);
    eprintln!("Map collection finished, now running orthogonal pathing algorithm");

    //Call functions to turn the edges into orthogonal paths and then turn those into corridors
    let orthogonal_edges = orthogonal_paths(edges, map);
    let corridors = create_corridors(orthogonal_edges);

    //Looks a little weird, v is just an index used inside the box function
    
    for cor in corridors {
        //v here is a counter that is used for the OBJ export
        v = add_obj_box(cor.0, cor.1, &mut obj_data, v);
    }


    let mut file = File::create("grch_export.obj").expect("Failed to create file");
    file.write_all(obj_data.as_bytes()).expect("Failed to write to file");
    println!("OBJ file exported");

}