#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    fn distance(&self, other: &Point) -> f32 {
        let x_off = other.x - self.x;
        let y_off = other.y - self.y;

        (x_off.powi(2) + y_off.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Line { // y = mx + c
    m: f32, // slope
    c: f32  // y offset
}

impl Line {
    fn from_points(p1: &Point, p2: &Point) -> Self {
        let m = (p2.y - p1.y) / (p2.x - p1.x);
        let c = p1.y - (p1.x * m);

        Line {
            m,
            c
        }
    }

    fn calc_y(&self, x: f32) -> f32 {
        self.m*x + self.c
    }

    fn calc_x(&self, y: f32) -> f32 {
        (y - self.c) / self.m
    }

    fn intersect(&self, other: &Line) -> Result<Point, &str> {
        if self.m == other.m && self.c == other.c {
            return Err("Lines are equal, overlapping")
        }

        if self.m == other.m && self.c != other.c {
            return Err("Lines are parallel, they do not overlap")
        }

        let x = (other.c - self.c) / (self.m - other.m);
        let y = self.m*x + self.c;

        let p = Point {
            x,
            y
        };

        Ok(p)
    }
}

fn main() {
    let origin = Point::origin();

    println!("The origin point is {:#?}", origin);
    
    let p = Point {
        x: 2.0,
        y: 1.0,
    };

    println!("Distance to p {}", origin.distance(&p));

    let l = Line::from_points(&origin, &p);

    println!("A line from origin to p: {:#?}", l);
    println!("Line y at x=10 is: {}", l.calc_y(10.0));
    println!("Line x at y=5 is: {}", l.calc_x(5.0));

    let l2 = Line {
        m: -2.0,
        c: 4.0
    };

    let intersect = l.intersect(&l2);

    match intersect {
        Ok(p) => println!("The lines intersect at: {:?}", p),
        Err(err) => println!("Lines do not intersect: {err}")
    }
}
