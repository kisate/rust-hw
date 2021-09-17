mod lib;
use crate::lib::{Circle, Point, Rect};

fn main() {
    let c = Circle { x: 0.0, y: 0.0, r: 3.0};
    let r = Rect {x: 0.0, y: 0.0, w: 3.0, h: 4.0};
    println!("{}", c.area());
    println!("{}", r.area());
    println!("{}", c.contains(&Point {x: 1.0, y: 1.0}));
    println!("{}", r.contains(&Point {x: 100.0, y: 100.0}))
}
