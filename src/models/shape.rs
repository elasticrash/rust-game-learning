pub trait Shape {
    fn vectors(&self) -> [[f64; 2]; 8];
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn new(x: f64, y: f64) -> Self;
}
