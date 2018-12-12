use std::env;

extern crate log;
use log::{info, trace, warn};

fn main() {
        let args: Vec<String> = env::args().collect();
        trace!("Then length of arguments is {}!", args.len());
        if args.len() != 3 {
                panic!("Too less arguments!");
        };
        trace!("The arguments is `{:?}`", args);

        let options = Options {
                pattern: &args[1],
                filename: &args[2],
        };
        trace!("The argument pattern is {}!", options.pattern);
        trace!("The argument filename is {}!", options.filename);
}

struct Options<'a> {
        pattern: &'a String,
        filename: &'a String,
}
