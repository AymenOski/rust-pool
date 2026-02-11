//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point(x, y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        PI * (self.radius * self.radius)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) <= self.radius + other.radius
    }
}

/*
    * Q & A :
    * Q1 : the Pythagorean theorem?
    - A1 : the distance between two points (x1, y1) and (x2, y2) can be calculated using the formula: distance = sqrt((x2 - x1)^2 + (y2 - y1)^2).
    * why? ; i will tell u why!
    a^2 + b^2 = c^2
    where a and b are the lengths of the legs of a right triangle, and c is the length of the hypotenuse. In our case, the hypotenuse is the distance between the two points.
    * Q2 : what is pi?
    - A2 : pi is a mathematical constant that represents the ratio of a circle's circumference to its diameter. It is approximately equal to 3.14159.
    the circumference of a circle is pi*r + pi*r = 2*pi*r ; the relation between a curve and a straight line is pi.
    so the circumference of a half circle is pi*r and for a full circle is 2*pi*r. -> halfe circle `(` and the other half `(` ; pi * r kinda like saying 180 degree is pi and 360 degree is 2*pi.
    * Q3 : what is the area of a circle?
    - A3 : the area of a circle is given by the formula: area = pi * r^2, where r is the radius of the circle. This formula can be derived using calculus, but it can also be understood intuitively by thinking about how the area of a circle relates to the area of a square using polygons. 
    * Q4 : for the intersect function, how do we determine if two circles intersect?
    - A4 : two circles intersect if the distance between their centers is less than or equal to the sum of their radii. This is because if the distance between the centers is greater than the sum of the radii, then the circles are too far apart to touch each other. If the distance is equal to the sum of the radii, then the circles are tangent to each other, meaning they touch at exactly one point. If the distance is less than the sum of the radii, then the circles overlap, meaning they intersect at two points.
    if the distance between the centers is less than the absolute difference of the radii, then one circle is completely inside the other without touching, and they do not intersect.
    if the distance between the centers is equal to the absolute difference of the radii, then one circle is tangent to the other from the inside, meaning they touch at exactly one point.
    if the distance between the centers is greater than the absolute difference of the radii but less than the sum of the radii, then the circles intersect at two points.
*/