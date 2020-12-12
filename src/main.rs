// precision dice
extern crate rand;

use rand::Rng;
use std::env;

fn main() {
    // get arguments for command line
    let mut args: std::env::Args = env::args();

    if args.len() < 2 {
        // default 1 ~ 6
        println!("{}", rand::thread_rng().gen_range(1, 7));
    } else {
        // first argument is program name
        args.next();

        if let Some(s) = args.next() {
            if let Ok(num) = s.parse::<i32>() {
                println!("{}", rand::thread_rng().gen_range(1, num + 1));
            } else {
                // can't convert i32
                println!("{}", rand::thread_rng().gen_range(1, 7));
            }
        }
    }
}
