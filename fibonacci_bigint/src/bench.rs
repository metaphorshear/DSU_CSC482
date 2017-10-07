extern crate num;
extern crate time;

use num::bigint::BigUint;

pub fn timed_run_ns(f: &Fn(u64) -> BigUint, arg: u64) -> u64 {
    let start = time::precise_time_ns();
    let result = f(arg);
    let end = time::precise_time_ns();
    //eprintln!("Result: {}, Time taken in ns: {}", result, end-start);
    end-start
}


