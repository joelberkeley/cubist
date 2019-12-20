use crate::two_dimensional::{Shape2D, Circle};

fn main() {
    let circle = Circle{radius: 17};

    for line in circle.draw(){
        println!("{}", line)
    }
}

type Coord = i32;

mod two_dimensional {
    use super::Coord;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Point2D{pub x: Coord, pub y: Coord}

    impl Point2D {
        pub fn neighbours(&self) -> Vec<Point2D> {
            [
                (-1,  1), (0,  1), (1,  1),
                (-1,  0),          (1,  0),
                (-1, -1), (0, -1), (1, -1)
            ].iter().map(
                |diff| Point2D{x: self.x + diff.0, y: self.y + diff.1}
            ).collect()
        }
    }

    pub trait Shape2D {
        /// Given a point anywhere on the x-y plane, return a measure of how far from the curve the
        /// point is.
        fn err(&self, point: Point2D) -> i64;

        /// Find the closest y value for a given value of x. If more than one value is returned,
        /// chooses one at random.
        fn get_y(&self, x: Coord) -> Option<Coord>;

        fn trace(&self) -> Vec<Point2D>;
        fn draw(&self) -> Vec<String>;
    }

    pub struct Circle {
        pub radius: Coord
    }

    impl Shape2D for Circle {
        fn err(&self, point: Point2D) -> i64 {
            let err_ = point.x.pow(2) + point.y.pow(2) - self.radius.pow(2);
            err_ as i64
        }

        fn get_y(&self, x: Coord) -> Option<Coord> {
            let y_squared = self.radius.pow(2) as f64 - x.pow(2) as f64;
            if y_squared < 0_f64 {
                None
            } else {
                Some(y_squared.sqrt().round() as Coord)
            }
        }

        fn trace(&self) -> Vec<Point2D> {
            let start_at = Point2D{x: 0, y: self.get_y(0).unwrap()};
            let mut curve = Vec::new();
            let mut current = start_at;
            let mut last: Option<Point2D> = None;

            loop {
                curve.push(current);

                let next = current.neighbours()
                    .iter()
                    .filter(|&p| match last {
                        Some(p_) => *p != p_,
                        _ => true
                    })
                    .map(|&point| (point, self.err(point)))
                    .min_by_key(|(_, err)| err.abs())  // takes the first minimum, not all of them
                    .unwrap().0;

                if next == start_at {
                    break
                }

                last = Some(current);
                current = next;
            }

            curve
        }

        fn draw(&self) -> Vec<String> {
            let canvas_width = 2 * self.radius + 1;
            let canvas_height = canvas_width;

            let mut pixels = vec!["."; (canvas_height * canvas_width) as usize];

            for point in self.trace() {
                // todo how to decouple self.radius from algorithm here?
                let idx = (self.radius + point.x) + canvas_width * (self.radius - point.y);
                pixels[idx as usize] = "o"
            }

            pixels.chunks(canvas_width as usize).map(|v| v.concat()).collect()
        }
    }
}
