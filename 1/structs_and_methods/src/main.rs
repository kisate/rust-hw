struct Point {
    x: f32,
    y: f32
}

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        p.x > self.x && p.x < self.x + self.w && p.y > self.y && p.y < self.y + self.h
    }

    fn area(&self) -> f32 {
        self.w * self.h
    }
 }

struct Circle {
    x: f32,
    y: f32,
    r: f32
}

impl Circle {
    fn contains(&self, p: &Point) -> bool {
        (p.x - self.x).powi(2) + (p.y - self.y).powi(2) < self.r.powi(2)
    }

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.r.powi(2)
    }
}


enum Figure {
    Rect(Rect),
    Circle(Circle)
}

impl Figure {
    fn contains(&self, p: &Point) -> bool {
        match self {
            Figure::Rect(fig) => fig.contains(p),
            Figure::Circle(fig) => fig.contains(p)
        }
    }

    fn area(&self) -> f32 {
        match self {
            Figure::Circle(fig) => fig.area(),
            Figure::Rect(fig) => fig.area()
        }
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, r: 3.0};
    let r = Rect {x: 0.0, y: 0.0, w: 3.0, h: 4.0};
    println!("{}", c.area());
    println!("{}", r.area());
    println!("{}", c.contains(&Point {x: 1.0, y: 1.0}));
    println!("{}", r.contains(&Point {x: 100.0, y: 100.0}))
}
