pub trait Shape {
    fn vectors(&self) -> [[f64; 2]; 8];
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn new(x: f64, y: f64) -> Self;
    fn boundingbox(&self) -> [[f64; 2]; 2] {
        let mut x_min = self.vectors()[0][0];
        let mut x_max = self.vectors()[0][0];
        let mut y_min = self.vectors()[0][1];
        let mut y_max = self.vectors()[0][1];

        for point in &self.vectors() {
            if x_min > point[0] {
                x_min = point[0];
            }
            if x_max < point[0] {
                x_max = point[0];
            }

            if y_min > point[1] {
                y_min = point[1];
            }
            if y_max < point[1] {
                y_max = point[1];
            }
        }

        return [[x_min, y_min], [y_min, y_max]];
    }
}
