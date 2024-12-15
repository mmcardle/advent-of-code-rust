use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq)]
pub struct Lattice {
    real: i64,
    imag: i64,
}

impl Lattice {
    pub const fn new(real: i64, imag: i64) -> Self {
        Self { real, imag }
    }

    pub fn x(&self) -> i32 {
        self.real as i32
    }

    pub fn y(&self) -> i32 {
        self.imag as i32
    }

    pub fn is_real(&self) -> bool {
        self.imag == 0
    }

    pub fn is_imag(&self) -> bool {
        self.real == 0
    }

    pub fn direction(&self, other: &Self) -> Lattice {
        let diff = *other - *self;
        let mut result = Lattice::new(0, 0);
        if diff.real > 0 {
            result.real = 1;
        } else if diff.real < 0 {
            result.real = -1;
        }
        if diff.imag > 0 {
            result.imag = 1;
        } else if diff.imag < 0 {
            result.imag = -1;
        }
        result
    } 

    pub fn to_point(&self) -> Point {
        Point {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x(), self.y())
    }

    pub fn around(&self, include_self: bool) -> Vec<Self> {
        let mut result = Vec::new();
        if include_self {
            result.push(*self);
        }
        for (r, i) in &Self::ALL_DIRS {
            result.push(Self::new(self.real + r, self.imag + i));
        }
        result
    }

    pub fn across(&self, include_self: bool) -> Vec<Self> {
        let mut result = Vec::new();
        if include_self {
            result.push(*self);
        }
        for (r, i) in &Self::FOUR_DIRS {
            result.push(Self::new(self.real + r, self.imag + i));
        }
        result
    }

    pub fn manhattan(a: Self, b: Self) -> i32 {
        (a.x() - b.x()).abs() + (a.y() - b.y()).abs()
    }

    pub fn from_coords(xy: (i64, i64)) -> Self {
        Self::new(xy.0, xy.1)
    }

    pub fn lattice_map<T, F>(data: &[Vec<T>], func: F) -> HashMap<Self, T>
    where
        T: Clone,
        F: Fn(&T) -> T,
    {
        let mut map = HashMap::new();
        for (y, row) in data.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                map.insert(Self::new(x as i64, y as i64), func(val));
            }
        }
        map
    }

    pub fn draw_lattice_map<T: fmt::Display>(map: &HashMap<Self, T>, fillvalue: &str, print_sep: bool) {
        let min_x = map.keys().map(|p| p.x()).min().unwrap();
        let max_x = map.keys().map(|p| p.x()).max().unwrap();
        let min_y = map.keys().map(|p| p.y()).min().unwrap();
        let max_y = map.keys().map(|p| p.y()).max().unwrap();
        for r in min_y..=max_y {
            for c in min_x..=max_x {
                let x = map.get(&Self::new(c as i64, r as i64));
                if x.is_none() {
                    print!(" {:>2} ", fillvalue);
                } else {
                    print!(" {:>2} ", x.unwrap());
                }
            }
            println!();
        }
        if print_sep {
            println!();
        }
    }

    pub const FOUR_DIRS: [(i64, i64); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    pub const ALL_DIRS: [(i64, i64); 8] = [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
    ];
}

impl Add for Lattice {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Sub for Lattice {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.real - other.real, self.imag - other.imag)
    }
}

impl Mul for Lattice {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

impl Neg for Lattice {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.real, -self.imag)
    }
}

impl PartialOrd for Lattice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_tuple().partial_cmp(&other.to_tuple())
    }
}

impl Ord for Lattice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_tuple().cmp(&other.to_tuple())
    }
}

impl Eq for Lattice {
    fn assert_receiver_is_total_eq(&self) {}
}

pub const N: Lattice = Lattice::new(0, -1);
pub const S: Lattice = Lattice::new(0, 1);
pub const E: Lattice = Lattice::new(1, 0);
pub const W: Lattice = Lattice::new(-1, 0);
pub const NE: Lattice = Lattice::new(1, -1);
pub const NW: Lattice = Lattice::new(-1, -1);
pub const SE: Lattice = Lattice::new(1, 1);
pub const SW: Lattice = Lattice::new(-1, 1);

// const ROT_90_CW: Lattice = Lattice::new(0, 1);
// const ROT_90_CCW: Lattice = Lattice::new(0, -1);

lazy_static::lazy_static! {
  static ref DIRECTIONS_MAP: HashMap<&'static str, Lattice> = {
    let mut m = HashMap::new();
    m.insert("N", N);
    m.insert("S", S);
    m.insert("E", E);
    m.insert("W", W);
    m.insert("NE", NE);
    m.insert("NW", NW);
    m.insert("SE", SE);
    m.insert("SW", SW);
    m
  };
  static ref DIRECTIONS: HashSet<Lattice> = {
    let mut s = HashSet::new();
    s.insert(N);
    s.insert(S);
    s.insert(E);
    s.insert(W);
    s.insert(NE);
    s.insert(NW);
    s.insert(SE);
    s.insert(SW);
    s
  };
}
