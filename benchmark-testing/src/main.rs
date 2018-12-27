#![feature(test)]

fn main() {
        println!("Hello, world!");
}


extern crate test;

use std::cmp;

#[derive(PartialOrd)]
#[derive(PartialEq)]
pub struct Point {
        x: i32,
        y: i32,
}

use std::ops;
use std::iter;

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

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

/*
impl ops::Range<Point> {
        
}
*/

#[cfg(test)]
mod tests {
        use super::*;
        use test::Bencher;

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
                        (Point {x: 0, y: 0}..Point {x: 1000, y: 1000}).fold(Point {x:0, y:0}, |a, b| {a + b});
                });
        }
}
