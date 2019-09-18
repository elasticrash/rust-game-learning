pub mod beam;
pub mod rock;
pub mod shape;
pub mod ship;
use rock::Rock as SpaceRock;
use ship::Ship as SpaceShip;

impl shape::Shape for SpaceRock {
    fn vectors(&self) -> [[f64; 2]; 8] {
        return [
            [8.0, -2.0],
            [21.0, 0.0],
            [36.0, -4.0],
            [32.0, -13.0],
            [30.0, -20.0],
            [25.0, -22.0],
            [10.0, -20.0],
            [8.0, -2.0],
        ];
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.x
    }

    fn new(x: f64, y: f64) -> SpaceRock {
        SpaceRock { x: x, y: y }
    }
}

impl shape::Shape for SpaceShip {
    fn vectors(&self) -> [[f64; 2]; 8] {
        return [
            [0.0, 0.0],
            [40.0, 0.0],
            [30.0, -10.0],
            [30.0, -20.0],
            [20.0, -30.0],
            [10.0, -20.0],
            [10.0, -10.0],
            [0.0, 0.0],
        ];
    }
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.x
    }

    fn new(x: f64, y: f64) -> SpaceShip {
        SpaceShip { x: x, y: y }
    }
}
