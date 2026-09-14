use std::ops::*;
use std::sync::atomic::Ordering;
use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;
use crate::SEED;
use crate::INDEX;
use crate::ROOM_CHANCE;
use std::convert::TryFrom;


#[derive(Debug)]
#[derive(PartialEq)]
pub struct BSPNode<T> {
    pub value: T,
    pub left: Option<Box<BSPNode<T>>>,
    pub right: Option<Box<BSPNode<T>>>,
    pub split_d: Axis
}





impl BSPNode<Tile> {
    //Refactor WIP: Point2 -> Point3, creating new splitting algorithm


    pub fn split(&mut self, rng: &mut StdRng) {
            let rng_factor = rng.random_range(0.3..0.7);

            let next_split = match self.split_d {
                            Axis::X => {
                                if self.value.get_depth() > self.value.get_height() {
                                    Axis::Z
                                } else {
                                    Axis::Y
                                }}
                            Axis::Y => {
                                if self.value.get_depth() > self.value.get_width() {
                                    Axis::Z
                                } else {
                                    Axis::X
                                }}
                            Axis::Z => {
                                if self.value.get_width() > self.value.get_height() {
                                    Axis::X
                                } else {
                                    Axis::Y
                                }
                            }
                        };
            
            match self.left  {
                Some(_) => (),
                None => {
                    self.left = Some(Box::from(BSPNode{
                        value: Tile {
                            index: INDEX.fetch_add(1, Ordering::Relaxed),
                            lc: self.value.lc,
                            rc: { 
                                if self.split_d == Axis::Z {
                                    Point3(self.value.rc.0, self.value.rc.1, self.value.rc.2 - (self.value.get_depth() as f64 * rng_factor) as i64)
                                } else if self.split_d == Axis::Y {
                                    Point3(self.value.rc.0, self.value.rc.1 - (self.value.get_height() as f64 * rng_factor) as i64, self.value.rc.2 )
                                } else {
                                    Point3(self.value.rc.0 - (self.value.get_width() as f64 * rng_factor) as i64, self.value.rc.1, self.value.rc.2)
                                }
                            },
                            traversible: rng.random_bool(ROOM_CHANCE),
                            split_count: (self.value.split_count + 1),
                            room: None
                        },
                        left:  None,
                        right: None,
                        split_d: next_split.clone()
                    }));
                    
                self.right = Some(Box::from(BSPNode{
                        value: Tile { 
                            index: INDEX.fetch_add(1, Ordering::Relaxed),
                            lc: {
                                if self.split_d == Axis::Z {
                                    Point3(self.value.lc.0, self.value.lc.1, self.left.as_ref().unwrap().value.rc.2)
                                } else if self.split_d == Axis::Y {
                                    Point3(self.value.lc.0, self.left.as_ref().unwrap().value.rc.1, self.value.lc.2)
                                } else {
                                    Point3(self.left.as_ref().unwrap().value.rc.0, self.value.lc.1, self.value.lc.2)
                                }
                            },
                            rc: self.value.rc,
                            traversible: rng.random_bool(ROOM_CHANCE),
                            split_count: (self.value.split_count + 1),
                            room: None
                        },
                        left:  None,
                        right: None,
                        split_d: next_split
                }));
            }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z 
}

impl Axis {
    pub fn random_variant() -> Axis {
        let mut rng = StdRng::seed_from_u64(SEED);
        return match rng.random_range(1..4) {
            1..2 => Axis::X,
            2..3 => Axis::Y,
            3..4 => Axis::Z,
            _ => Axis::X
        }
    }
}

impl TryFrom<i32> for Axis {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Axis::X),
            1 => Ok(Axis::Y),
            2 => Ok(Axis::Z),
            _ => Err("Invalid axis value"),
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]

pub struct Tile {
    pub index: usize,
    pub lc: Point3,
    pub rc: Point3,
    pub traversible: bool,
    pub split_count: u32,
    pub room: Option<Room>
}

impl Tile {
    pub fn get_depth(&self) -> i64 {
        return self.rc.2 - self.lc.2
    }
    pub fn get_height(&self) -> i64 {
        return self.rc.1 - self.lc.1
    }
    pub fn get_width(&self) -> i64 {
        return self.rc.0 - self.lc.0
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]

pub struct Point3(pub i64, pub i64, pub i64);

impl Point3 {
    pub fn sum(&self) -> i64 {
        return &self.0 + &self.1 + &self.2
    }
    pub fn add_at(&mut self, index: usize, value: i64) {
        self[index] += value;
    }
    pub fn set_to(&mut self, index: usize, value: i64) {
        self[index] = value;
    }
}

impl std::ops::Index<usize> for Point3 {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Point3 index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<usize> for Point3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Point3 index out of bounds"),
        }
    }
}

impl Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Room(pub usize, pub Point3, pub Point3, pub bool);