extern crate time;

pub fn timed_run_ns<T,U>(f: &Fn(T) -> U, arg: T) -> (U,u64) {
    let start = time::precise_time_ns();
    let result = f(arg);
    let end = time::precise_time_ns();
    //eprintln!("Result: {}, Time taken in ns: {}", result, end-start);
    (result, end-start)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum(v: &Vec<u32>)->u32 {
        /// This is just to test the timed_run_ns function
        v.iter().fold(0, |acc, &x| acc + x)
    }

    #[test]
    fn timed_run_takes_vec_ref(){
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (res, time) = timed_run_ns(&sum, &v);
        eprintln!("Result: {}\nTime taken: {} ns", res, time);
    }
}
