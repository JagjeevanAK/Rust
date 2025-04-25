enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape:Shape)-> f64{
    match shape{
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Rectangle(width, length) => length * width
    }
}

fn main() {
    let circle = Shape::Circle(3.04);
    let square = Shape::Square(4.0);
    let rect = Shape::Rectangle(4.0,5.0);

    println!("{}",calculate_area(circle));
    println!("{}",calculate_area(square));
    println!("{}",calculate_area(rect));
}
