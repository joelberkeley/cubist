use crate::two_dimensional::{Shape2D, Point2D, Circle};

fn main() {
    let circle = Circle{radius: 5};

//    for point in circle.get_boundary() {
//        println!("{:?}", point);
//    }

    for point in &circle.trace(Point2D{x: 0, y: circle.radius}) {
        println!("{:?}", point);
    }

    for line in circle.draw(){
        println!("{}", line)
    }
}

type Coord = i16;

mod two_dimensional {
    use super::Coord;
    use std::collections::HashSet;
    use itertools::Itertools;

    #[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
    pub struct Point2D{pub x: Coord, pub y: Coord}

    pub trait Shape2D {
//        fn err(&self) -> Box<dyn Fn(Point2D) -> f64>;
        fn get_boundary(&self) -> Vec<Point2D>;
        fn draw(&self) -> Vec<String>;
    }

    pub struct Circle {
        pub radius: Coord
    }

    impl Circle {
        pub fn trace(&self, start_at: Point2D) -> HashSet<Point2D> {
            let mut curve = HashSet::new();

            fn get_neighbours(current: Point2D, excluding: HashSet<Point2D>) -> HashSet<Point2D> {
                let mut points: HashSet<Point2D> = (-1..=1).permutations(2).map(
                    |v| Point2D{x: current.x + v[0], y: current.y + v[1]}
                ).collect();

                points.remove(&current);
                if last.is_some() {
                    points.remove(&last.unwrap());
                }

                points
            }

            let (mut current, mut last) = (start_at, None);
            loop {
                curve.insert(current);

                let next = *get_neighbours(current, last)
                    .iter()
                    .map(
                        |point| (point, point.x.pow(2) + point.y.pow(2) - self.radius.pow(2))
                    )
                    .min_by_key(|(_, err)| err.abs())  // takes the first minimum, not all of them
                    .unwrap().0;

                println!("curr {:?}", current);
//                if last.is_some() {println!("last {}", last.unwrap());}
//                println!("next {}", next);
                if next == start_at {
                    break
                }

                last = Some(current);
                current = next;
            }

            curve
        }

        pub fn get_boundary_map(&self) -> Vec<(Coord, Vec<Coord>)> {
            fn get_x_values(y: Coord, radius: Coord) -> Vec<Coord> {
                let squared_diff = radius.pow(2) - y.pow(2);
                if squared_diff >= 0_i16 {
                    let x = (squared_diff as f64).sqrt().round() as Coord;
                    if x == 0 {vec![x]} else {vec![x, -x]}
                } else {
                    vec![]
                }
            }

            let radius = self.radius;

            (-radius..=radius)
                .into_iter()
                .map(|y| (y, get_x_values(y, radius)))
                .collect::<Vec<(Coord, Vec<Coord>)>>()
        }
    }

    impl Shape2D for Circle {
        fn get_boundary(&self) -> Vec<Point2D> {
            fn get_x_values(y: Coord, radius: Coord) -> Option<Vec<Point2D>> {
                let squared_diff = (radius as f64).powi(2) - (y as f64).powi(2);
                if squared_diff >= 0_f64 {
                    let x = squared_diff.sqrt().round() as Coord;
                    if x == 0 {
                        Some(vec![Point2D{x, y}])
                    } else {
                        Some(vec![Point2D{x, y}, Point2D{x: -x, y}])
                    }
                } else {
                    None
                }
            }

            let radius = self.radius;

            (-radius..=radius)
                .filter_map(|y| get_x_values(y, radius))
                .collect::<Vec<Vec<Point2D>>>()
                .concat()
        }

        fn draw(&self) -> Vec<String> {
            let boundary = self.get_boundary_map();

            let radius = self.radius;

            fn mark_boundary<'a>(x: i16, xs: &Vec<i16>, y: &i16) -> String {
                (if xs.contains(&x) {"x".to_owned()} else {".".to_owned()})
            }

            boundary.iter().map(
                |(y, xs)| (-radius..=radius)
                    .map(|x| mark_boundary(x, xs, y))
                    .collect::<Vec<String>>()
                    .concat()
            )
                .collect::<Vec<String>>()

//            let mut line = " ".to_owned().repeat(2 * *self.radius);
//            line.push_str("\n");
//            let line = line;
        }
    }
}

mod three_dimensional {
    use super::Coord;

    #[derive(Copy, Clone)]
    struct Point3D(Coord, Coord, Coord);

    struct Sphere {
        radius: u16
    }
}
