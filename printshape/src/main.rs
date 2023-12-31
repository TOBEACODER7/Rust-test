trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side_length: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("这个图形的面积是: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 4.0, height: 3.0 };
    let square = Square { side_length: 6.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}
