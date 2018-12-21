
/*
// using in function
fn max<T> (v: &[T]) -> Result<T, i32> {
        if v.len() == 0 {
                return Err(-1);
        }
        let mut maximum = v[0];
        for &item in v.iter() {
                if maximum < item {  // unimplement trait std::cmp::PartialOrd
                        maximum = item;
                }
        }
        Ok(maximum)
}
*/

fn main() {
/*
        let v = vec![-33, 2, 55, 0, -3];
        println!("The maximum is {:?}!", max(&v));
*/

        let p = Point {x:3, y:3.3};
        println!("{:?}", p);

        let p2 = Point {x:4, y:2.2};
        println!("{:?}", p.transfer(p2));
}

// generic for struct
#[derive(Debug)]
struct Point<Tx, Ty> {
        x: Tx,
        y: Ty,
}

// using in method
impl<Tx, Ty> Point<Tx, Ty> {
        fn transfer(self, other: Point<Tx, Ty>) -> Point<Tx, Ty> {
                Point {
                        x: self.x,
                        y: other.y,
                }
        }
}

// generic for enum
/*
enum Result<T, E> {
        Ok(T),
        Err(E),
}
*/
