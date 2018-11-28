fn main() {
        // integer i/u-8,16,32,64,128
        let i = 3;  // i32 decimal
        let h = 0x33;  // hex
        let o = 0o33;  // octo
        let b = 0b11;  // binary
        println!("{}", i);
        println!("{}", h);
        println!("{}", o);
        println!("{}", b);

        // float f32/f64
        let f = 3.3;  // f64
        let e = 3e3;  // scientific notation
        println!("{}", f);
        println!("{}", e);

        // char (unicode)
        let asc = 'A';  // ascii
        let zh  = '中';  // zh-cn
        println!("{}", asc);
        println!("{}", zh);

        // bool
        let t = true;
        println!("{}", t);
}
