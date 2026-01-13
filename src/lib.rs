// src/lib.rs

// ---------- 1. BASICS ----------
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn flip(b: bool) -> bool {
    !b
}

// ---------- 2. OWNERSHIP & BORROWING ----------
pub fn take_ownership(s: String) -> usize {
    s.len()
}

pub fn borrow_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn push_exclamation(s: &mut String) {
    s.push('!');
}

// ---------- 3. STRUCTS, ENUMS, METHODS ----------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { center: Point, radius: f64 },
    Rect { top_left: Point, w: f64, h: f64 },
    Triangle { a: Point, b: Point, c: Point },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius, .. } => std::f64::consts::PI * radius * radius,
            Shape::Rect { w, h, .. } => w * h,
            Shape::Triangle { a, b, c } => {
                let area = a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y);
                area.abs() * 0.5
            }
        }
    }
}

// ---------- 4. TRAITS & GENERICS ----------
pub trait Plottable {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Plottable for Point {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
}

impl Plottable for (f64, f64) {
    fn x(&self) -> f64 { self.0 }
    fn y(&self) -> f64 { self.1 }
}

// Return a reference to the item farthest from the origin.
// Note the explicit lifetime tying the returned reference to the input slice.
pub fn furthest_from_origin<T: Plottable>(items: &[T]) -> Option<&T> {
    items.iter().max_by(|a, b| {
        let da = a.x() * a.x() + a.y() * a.y();
        let db = b.x() * b.x() + b.y() * b.y();
        da.partial_cmp(&db).unwrap()
    })
}

pub fn min_by_key<T, K, F>(items: &[T], mut f: F) -> Option<&T>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    let mut iter = items.iter();
    let mut min = iter.next()?;
    let mut min_key = f(min);

    for item in iter {
        let key = f(item);
        if key < min_key {
            min = item;
            min_key = key;
        }
    }
    Some(min)
}

// ---------- 5. ERRORS & OPTION/RESULT ----------
pub fn parse_port(s: &str) -> Result<u16, String> {
    s.parse::<u16>().map_err(|e| format!("Invalid port '{}': {}", s, e))
}

// ---------- 6. ITERATORS & CLOSURES ----------
pub fn even_squares(n: u32) -> Vec<u32> {
    (0..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

// ---------- 7. USING A CRATE (rand) ----------
pub fn roll_dice(sides: u8) -> u8 {
    use rand::Rng;
    rand::thread_rng().gen_range(1..=sides)
}
