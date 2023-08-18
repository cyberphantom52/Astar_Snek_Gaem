pub type Map = Vec<Vec<Block>>;

macro_rules! wrap_x {
    ($x:expr) => {
        (($x % GRID_COLS) + GRID_COLS) % GRID_COLS
    };
}

macro_rules! wrap_y {
    ($y:expr) => {
        (($y % GRID_ROWS) + GRID_ROWS) % GRID_ROWS
    };
}

pub (crate) use wrap_x;
pub (crate) use wrap_y;


#[derive(Eq, Hash, Copy, Clone)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl core::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl core::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector {
    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Block {
    pub pos: Vector,
    pub parent_pos: Vector,
    pub is_closed: bool,
    pub is_wall: bool,
    pub g_val: u16,
    pub f_val: u16,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Vector { x, y },
            parent_pos: Vector { x: 0, y: 0 },
            is_closed: false,
            is_wall: false,
            g_val: 0,
            f_val: 0,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl std::ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}
