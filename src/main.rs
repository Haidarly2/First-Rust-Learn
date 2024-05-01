use std::f64::consts::PI;
enum Shape {
    Circle(f64),
    Triangle(f64, f64),
    Rectangle(f64, f64),
}
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(diameter) => { let radius = diameter / 2.0; PI * radius.powi(2) }
            Shape::Triangle(base, height) => { (base * height) / 2.0 }
            Shape::Rectangle(width, length) => { width * length }
        }
    }
}
fn main() {
    let base: f64 = 24.0;
    let height: f64 = 24.0;
    let triangle = Shape::Triangle(base, height);
    let triangle_area = triangle.area();
    let width: f64 = 12.0;
    let length: f64 = 24.0;
    let rectangle = Shape::Rectangle(width, length);
    let rectangular_area = rectangle.area();
    let diameter: f64 = 45.0;
    let circle = Shape::Circle(diameter);
    let circle_area = circle.area();

    println!(
        "The area of the triangle with a base of {} and a height of {} is {:.5}",
        base, height, triangle_area
    );
    println!(
        "The area of the rectangle with a width of {} and a length of {} is {:.5}",
        width, length, rectangular_area
    );
    println!(
        "The area of the circle with a diameter of {} is {:.5}",
        diameter, circle_area
    );
}