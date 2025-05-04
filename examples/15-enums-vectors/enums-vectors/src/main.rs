enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64), 
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(a,b,c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
    fn name(&self) -> String {
        match self {
            Shape::Circle(_) => "Circle".to_string(),
            Shape::Square(_) => "Square".to_string(),
            Shape::Triangle(_, _, _) => "Triangle".to_string(),
        }
    }
}

fn get_lagest_shape(shapes: &[Shape]) -> &Shape {
    let mut largest_shape = &shapes[0];
    for shape in shapes.iter() {
        if shape.area() > largest_shape.area() {
            largest_shape = shape;
        }
    }
    largest_shape
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(3.0, 4.0, 5.0)];
    let largest_shape = get_lagest_shape(&shapes);
    println!("Largest shape is {} with area: {}", largest_shape.name(), largest_shape.area());
}
