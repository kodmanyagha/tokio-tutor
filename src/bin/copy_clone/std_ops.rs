use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        println!("add function invoked.");

        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        println!("add_assign function invoked.");

        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        println!("div function invoked.");

        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        println!("mul function invoked.");

        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

pub fn std_ops1() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);

    let mut p3 = p1 + p2;
    println!("p3: {:?}", p3);

    p3 += p2;
    p3 += p2;
    p3 += p2;
    println!("p3: {:?}", p3);

    let p4 = p3 * p2;
    println!("p4: {:?}", p4);

    let p5 = p4 / p2;
    println!("p5: {:?}", p5);
}
