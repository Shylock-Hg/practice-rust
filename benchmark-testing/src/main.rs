#![feature(test, step_trait)]

fn main() {
        println!("Hello, world!");
}

/*
#[derive(PartialOrd)]
#[derive(PartialEq)]
pub struct Point {
x: i32,
y: i32,
}
 */

// use std::cmp;
use std::ops;
// use std::iter;
// use std::ops::Range;
use std::iter::Step;
// use std::iter::Iterator;

extern crate test;

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub struct Point {
x: i32,
           y: i32,
}

impl ops::Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
                Point {
x: self.x + other.x,
           y: self.y + other.y,
                }
        }
}

impl Step for Point {
        // get max count of step
        fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                if start.x == start.y && end.x == end.y {
                        if end.x >= start.x {
                                return Some((end.x - start.x) as usize);
                        }
                }
                None
        }
        fn replace_one(&mut self) -> Self {
                std::mem::replace(self, Point{x:1, y:1})
        }
        fn replace_zero(&mut self) -> Self {
                std::mem::replace(self, Point{x:1, y:1})
        }
        fn add_one(&self) -> Self {
                let mut s = self.clone();
                s.x += 1;
                s.y += 1;
                s
        }
        fn sub_one(&self) -> Self {
                let mut s = self.clone();
                s.x -= 1;
                s.y -= 1;
                s
        }
        fn add_usize(&self, n: usize) -> Option<Self> {
                let mut s = self.clone();
                s.x += n as i32;
                s.y += n as i32;
                Some(s)
        }
}

/*
   struct Iter<T> {
pointer: T,
end: T,
}

impl Iterator for Iter<Point> {
type Item = Point;
fn next(&mut self) -> Option<Self::Item> {
self.pointer.x += 1;
self.pointer.y += 1;
let p = Point {
x: self.pointer.x,
y: self.pointer.y,
};
if p > self.end {
None
} else {
Some(p)
}
}
}
 */

/*
   impl ops::Range<Point> {

   }
 */

#[cfg(test)]
mod tests {
        use super::*;
        use test::Bencher;

        fn add_two(x: i32) -> i32 {
                x + 2
        }

        #[bench]
        fn bench_add_two(b: &mut Bencher) {
                b.iter(|| add_two(2));
        }

        #[bench]
        fn bench_xor_1000_ints(b: &mut Bencher) {
                b.iter(|| {
                        (0..1000).fold(0, |old, new| old ^ new);
                });
        }

        #[bench]
        fn test_struct(b: &mut Bencher) {
                let p = Point {
                        x: 1000,
                        y: 2000,
                };
                b.iter(|| {
                        let p = (Point {x: 0, y: 0}..Point {x: 1000, y: 1000}).fold(Point {x:0, y:0}, |a, b| {a + b});
                });
        }
}
