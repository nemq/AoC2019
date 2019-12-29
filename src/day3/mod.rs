use crate::day::Day;


pub struct Day3 {

}

impl Day3 {

   

    pub fn read_curves(&self) -> (Curve, Curve) {
       let path = self.first_input();
       let lines: Vec<String> = self.read_input_lines_string(&path);

       if lines.len() != 2 {
           panic!("input file must contain exactly two lines");
       }

       let c1 = self.parse_curve(&lines[0]);
       let c2 = self.parse_curve(&lines[1]);
       (c1, c2)
    }

    pub fn parse_curve(&self, line: &String) -> Curve {

       let tokens: Vec<String> = line.split(',').map(|t| t.trim().to_owned()).collect();

       let mut curve = Curve::new();
       let mut p0 = Point::new(0, 0);
       let mut p1: Point;
       let mut s: Segment;

       for t in tokens.iter().map(|t| t.as_str()) {
            let (d, n) = t.split_at(1);
            match (d, n.parse::<i32>().unwrap()) {
                ("U", n) => {
                    p1 = Point::new(p0.x, p0.y + n);
                    s = Segment::new(p0.clone(), p1.clone());
                    p0 = p1;
                }, 
                ("R", n) => {
                    p1 = Point::new(p0.x + n, p0.y);
                    s = Segment::new(p0.clone(), p1.clone());
                    p0 = p1;
                }, 
                ("D", n) => {
                    p1 = Point::new(p0.x, p0.y - n);
                    s = Segment::new(p0.clone(), p1.clone());
                    p0 = p1;
                }, 
                ("L", n) => {
                    p1 = Point::new(p0.x - n, p0.y);
                    s = Segment::new(p0.clone(), p1.clone());
                    p0 = p1;
                }, 
                _ => {
                    panic!(format!("invalid token: {}", t));
                }
            }

            curve.add_segment(s);
       }

       curve
    }
}

#[derive(PartialEq, Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn dist(&self, other: &Point) -> u32 {
        let dx = (other.x- self.x).abs();
        let dy = (other.y - self.y).abs();
        (dx + dy) as u32
    }
}

#[derive(PartialEq, Debug)]
pub struct Segment {
    start: Point,
    end: Point,
}

impl Segment {

    pub fn new(start: Point, end: Point) -> Segment {
        if start == end {
            panic!("invalid segment")
        }

        Segment{start, end}
    }


    pub fn orientation(&self) -> Orientation {
        if self.start.x == self.end.x {
            Orientation::Vertical
        } else if self.start.y == self.end.y {
            Orientation::Horizontal
        } else {
            panic!("invalid orientation")
        }
    }

    pub fn min_x(&self) -> i32 {
        i32::min(self.start.x, self.end.x)
    }

    pub fn max_x(&self) -> i32 {
        i32::max(self.start.x, self.end.x)
    }

    pub fn min_y(&self) -> i32 {
        i32::min(self.start.y, self.end.y)
    }

    pub fn max_y(&self) -> i32 {
        i32::max(self.start.y, self.end.y)
    }
    
    pub fn containes(&self, p: &Point) -> bool {
        match self.orientation() {
            Orientation::Horizontal => {
                if p.y == self.start.y && self.min_x() < p.x && p.x < self.max_x() {
                    true
                }
                else {
                    false
                }
            },

            Orientation::Vertical => {
                if p.x == self.start.x && self.min_y() < p.y && p.y < self.max_y() {
                    true
                }
                else {
                    false
                }
            }
        }
    }

    pub fn intersection_point(&self, other :&Segment) -> Option<Point> {
        match (self.orientation(), other.orientation()) {
            (Orientation::Vertical, Orientation::Vertical) => None,
            (Orientation::Horizontal, Orientation::Horizontal) => None,
            (Orientation::Vertical, Orientation::Horizontal) => {
                let candidate = Point::new(self.start.x, other.start.y);
                if self.containes(&candidate) && other.containes(&candidate) {
                    Some(candidate)
                } else {
                    None
                }
            }
            (Orientation::Horizontal, Orientation::Vertical) => {
                let candidate = Point::new(other.start.x, self.start.y);
                if self.containes(&candidate) && other.containes(&candidate) {
                    Some(candidate)
                } else {
                    None
                }
            }
        }
    }

    pub fn length(&self) -> u32 {
        self.start.dist(&self.end)
    }
}

pub struct Curve {
    segments: Vec<Segment>
}

impl Curve {
    pub fn new() -> Curve {
        Curve{segments: Vec::new()}
    }

    pub fn add_segment(&mut self, s: Segment) {
        self.segments.push(s)
    }

    pub fn intersection_points(&self, other: &Curve) -> Vec<Point> {
        let mut points = Vec::new();

        for self_seg in self.segments.iter() {
            for other_seg in other.segments.iter() {
                if let Some(p) = self_seg.intersection_point(other_seg) {
                    points.push(p)
                }
            }
        }

        points
    }

    pub fn steps_to_reach(&self, point: &Point) -> u32 {
        let mut count = 0;
        for s in self.segments.iter() {
            if s.containes(point) {
                count += s.start.dist(point);
                break;
            } else {
                count += s.length();
            }
        }

        count
    }
}



impl Day for Day3 {
    fn first_puzzle(&self) -> String {

        let (c1, c2) = self.read_curves();
        let intersections = c1.intersection_points(&c2);

        let origin = Point::new(0, 0);
        let mut min = u32::max_value();
        for p in intersections.iter() {
            min =u32::min(min, p.dist(&origin));
        }

        format!("{}", min)
    }

    fn second_puzzle(&self) -> String {

        let (c1, c2) = self.read_curves();
        let intersections = c1.intersection_points(&c2);

        let mut min = u32::max_value();
        for p in intersections.iter() {
            let s1 = c1.steps_to_reach(p);
            let s2 = c2.steps_to_reach(p);
            min = u32::min(min, s1 + s2);
        }

        format!("{}", min)
    }

    fn number(&self) -> u8 {
        3
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    static DAY3: Day3 = Day3 {};

    #[test]
    fn point_dist() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 3);
        assert_eq!(a.dist(&b), 6);
        assert_eq!(b.dist(&a), 6);

        let c = Point::new(5, 5);
        assert_eq!(a.dist(&c), 10);
        assert_eq!(c.dist(&a), 10);

        let e = Point::new(-2, 3);
        let d = Point::new(3, -2);
        assert_eq!(e.dist(&d), 10);
        assert_eq!(d.dist(&e), 10);
    }

    #[test]
    #[should_panic]
    fn segment_orientation() {

        let hs = Point::new(-10, 4);
        let he = Point::new(10, 4);
        let hseg = Segment::new(hs, he);
        assert_eq!(hseg.orientation(), Orientation::Horizontal);


        let vs = Point::new(4, -10);
        let ve = Point::new(4, 10);
        let vseg = Segment::new(vs, ve);
        assert_eq!(vseg.orientation(), Orientation::Vertical);

        let cv = Point::new(-10, -10);
        let ce = Point::new(10, 10);
        let cseg = Segment::new(cv, ce);
        cseg.orientation(); //should panic
    }

    #[test]
    fn segment_intersection() {

        let mut vseg = Segment::new(Point::new(4, 2), Point::new(4, -2));
        let mut hseg = Segment::new(Point::new(-2, -1), Point::new(5, -1));
        
        assert_eq!(vseg.intersection_point(&hseg), Some(Point::new(4, -1)));
        assert_eq!(vseg.intersection_point(&vseg), None);
        assert_eq!(hseg.intersection_point(&hseg), None);


        vseg = Segment::new(Point::new(6, 7), Point::new(6, 3));
        hseg = Segment::new(Point::new(0, 0), Point::new(8, 0));
        assert_eq!(vseg.intersection_point(&hseg), None);


        vseg = Segment::new(Point::new(8, 0), Point::new(8, 5));
        hseg = Segment::new(Point::new(6, 3), Point::new(2, 3));
        assert_eq!(vseg.intersection_point(&hseg), None);
    }

    #[test]
    fn parse_curve() {

        let line = String::from("R8,U5,L5,D3");
        let curve = DAY3.parse_curve(&line);
        assert_eq!(curve.segments.len(), 4);
        assert_eq!(curve.segments[0], Segment::new(Point::new(0, 0), Point::new(8, 0)));
        assert_eq!(curve.segments[3], Segment::new(Point::new(3, 5), Point::new(3, 2)));
    }


    #[test]
    fn curve_intersection() {
        let l0 = String::from("R8,U5,L5,D3");
        let c0 = DAY3.parse_curve(&l0);
        let l1 = String::from("U7,R6,D4,L4");
        let c1 = DAY3.parse_curve(&l1);

        let intersection = c0.intersection_points(&c1);
        assert_eq!(intersection.len(), 2);
        assert_eq!(intersection.contains(&Point::new(3,3)), true);
        assert_eq!(intersection.contains(&Point::new(6,5)), true);
    }

    #[test]
    fn steps_to_reach() {
        let l0 = String::from("R8,U5,L5,D3");
        let c0 = DAY3.parse_curve(&l0);
        let l1 = String::from("U7,R6,D4,L4");
        let c1 = DAY3.parse_curve(&l1);

        let i0 = Point::new(3, 3);
        assert_eq!(c0.steps_to_reach(&i0), 20); 
        assert_eq!(c1.steps_to_reach(&i0), 20); 
        
    }


}