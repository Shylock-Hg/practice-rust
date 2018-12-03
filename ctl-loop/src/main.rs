fn main() {
        //loop_test()
        //loop_break();
        //while_test();
        for_test();
}

fn loop_break() {
        let mut count = 0;
        let result = loop {
                count += 1;
                if count == 10 {
                        break count * 2;
                }
        };
        
        debug_assert_eq!(result, 20);
}

fn loop_test() {
        loop {
                println!("Hello!");
        }
}

fn while_test() {
        let mut count = 3;
        while count != 0 {
                println!("Count is {}!", count);
                count -= 1;
        }
}

fn for_test() {
        let arr = [1, 2, 3, 4];
        for a in arr.iter() {
                println!("{}", a);
        }

        for a in (1..5).rev() {
                println!("{}", a);
        }
}
