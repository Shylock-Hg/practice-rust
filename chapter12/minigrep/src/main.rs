use std::env;

extern crate log;
extern crate stderrlog;
use log::{trace, debug, info, warn, error};

use minigrep::Options;

fn main() {
        // stderrlog
        stderrlog::new().module(module_path!()).verbosity(99).init().unwrap();

        // options
        let options = Options::from(env::args());
        trace!("The argument pattern is `{}`!", options.pattern);
        trace!("The argument filename is `{}`!", options.filename);

        // open and read file
        minigrep::run(&options);
}
