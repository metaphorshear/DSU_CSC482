extern crate time;

pub fn timed_run_ns(f: &Fn(u64) -> u64, arg: u64) -> u64 {
    for _ in 0..100{
        let mut tmp = f(arg);
        tmp = tmp - 1; //just trying to keep it from being optimized away
    }
    let start = time::precise_time_ns();
    let result = f(arg);
    let end = time::precise_time_ns();
    //eprintln!("Result: {}, Time taken in ns: {}", result, end-start);
    end-start
}


