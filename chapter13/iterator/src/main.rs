fn main() {
        let v = vec![1, 2, 3];
        let itor = v.iter();
        for it in itor {  // assignment
                println!("Item is {:?}!", it);
        }
        let v = vec![1, 2, 3];
        let mut itor = v.iter();
        println!("Next is {:?}!", itor.next());
        println!("Next is {:?}!", itor.next());
        println!("Next is {:?}!", itor.next());
        println!("Next is {:?}!", itor.next());

        let v = vec![1, 2, 3];
        let itor = v.iter();
        let sum: i32 = itor.sum();
        assert_eq!(6, sum);

        // adaptor -- transform iterator to iterator
        let v = vec![1, 2, 3];
        let itor = v.iter();
        let v2: Vec<_> = itor.map(|x| x+1).collect();
        assert_eq!(v2, vec![2, 3, 4]);

        let v = vec![
                Shoe {size: 10, style: String::from("XYZ")},
                Shoe {size: 13, style: String::from("Playboy")},
                Shoe {size: 10, style: String::from("Zed")},
        ];
        let in_my_size = shoes_in_size(v, 10);
        assert_eq!(in_my_size,
                vec![
                        Shoe {size: 10, style: String::from("XYZ")},
                        Shoe {size: 10, style: String::from("Zed")},
                ]
        );

        let mut iter = Counter::new();
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(5), iter.next());
        assert_eq!(None, iter.next());

        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                .map(|(a, b)| a*b).filter(|a| a%3 == 0).sum();
        assert_eq!(18, sum);
}

#[derive(PartialEq, Debug)]
struct Shoe {
        size: u32,
        style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe>{
        // iter, into_iter and itor_mut -- 
        let itor = shoes.into_iter().filter(|shoe| shoe.size == size);
        let targets: Vec<Shoe> = itor.collect();
        targets
}

// implement Iterator
#[derive(Debug)]
struct Counter {
        count: u32
}

impl Counter {
        fn new() -> Counter {
                Counter {count: 0}
        }
}

impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < 6 {
                        Some(self.count)
                } else {
                        None
                }
        }
}
