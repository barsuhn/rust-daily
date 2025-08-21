
#[derive(Clone)]
struct Point {
    pub x: f64,
    pub y: f64
}

struct Line {
    pub start: Point,
    pub end: Point
}

struct Circle {
    pub center: Point,
    pub radius: f64
}

enum Shape {
    LineShape(Line),
    CircleShape(Circle)
}

pub fn print_shapes() {
    let points = [
        Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 }, Point { x: 1.0, y: 0.0 },
    ];

    let shapes = [
        Shape::LineShape(Line {start:points[1].clone(), end: points[2].clone()}),
        Shape::CircleShape(Circle { center: points[0].clone(), radius: 1.0})
    ];

    for shape in shapes {
        match shape {
            Shape::LineShape(l) =>
                println!("line: from ({}, {}) to ({}, {})", l.start.x, l.start.y, l.end.x, l.end.y),
            Shape::CircleShape(c) =>
                println!("circle: at ({}, {}) radius {}", c.center.x, c.center.y, c.radius),
        }
    }
}

