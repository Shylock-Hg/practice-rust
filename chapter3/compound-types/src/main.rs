fn main() {
        // tuple
        let tup:(i32, char, bool) = (3, 'A', false);
        println!("{}", tup.1);
        
        // array
        let arr:[char; 4] = ['A', 'b', 'C', 'd'];
        println!("{}", arr[3]);
        
        // tuple of array
        let tups = (3, arr);
        println!("{}", tups.1[3]);
        
        // array of tuple
        let arrs = [tup, (3, 'C', true)];
        println!("{}", arrs[0].1);
}
