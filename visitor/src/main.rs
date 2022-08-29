use std::f64::consts::PI;

trait ShapeVisitor {
    fn visit_circle(&mut self, circle: &Circle);
    fn visit_square(&mut self, square: &Square);
}

trait Shape {
    fn accept<V: ShapeVisitor>(&self, visitor: &mut V);
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn accept<V: ShapeVisitor>(&self, visitor: &mut V) {
        visitor.visit_circle(self);
    }
}

impl Shape for Square {
    fn accept<V: ShapeVisitor>(&self, visitor: &mut V) {
        visitor.visit_square(self);
    }
}

struct AreaCalculator {
    area: f64,
}

impl AreaCalculator {
    fn new() -> Self {
        AreaCalculator { area: 0.0 }
    }
    fn area(self) -> f64 {
        self.area
    }
}
impl ShapeVisitor for AreaCalculator {
    fn visit_circle(&mut self, circle: &Circle) {
        self.area = PI * circle.radius * circle.radius;
    }
    fn visit_square(&mut self, square: &Square) {
        self.area = square.side * square.side;
    }
}

fn compute_area<S: Shape>(shape: S) -> f64 {
    let mut area_calculator = AreaCalculator::new();
    shape.accept(&mut area_calculator);
    area_calculator.area()
}

fn main() {
    let square = Square { side: 9.8 };
    let circle = Circle { radius: 11.1 };

    let square_area = compute_area(square);
    println!("Square area {}", square_area);

    let circle_area = compute_area(circle);
    println!("Circle area {}", circle_area)
}

#[cfg(test)]
mod tests {
    use crate::{compute_area, Circle, Square};
    #[test]
    fn it_should_compute_square_area() {
        let square = Square { side: 3.0 };
        let square_area = compute_area(square);

        assert_eq!(square_area, 9.0);
    }
    #[test]
    fn it_should_compute_circle_area() {
        let circle = Circle { radius: 6.7 };
        let circle_area = compute_area(circle);

        assert_eq!(circle_area, 141.02609421964584);
    }
}
