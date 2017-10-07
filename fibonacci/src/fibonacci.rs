use matrix::Matrix;

pub fn fib_loop(n: u64) -> u64{
    if n <= 1 { 1 }
    else {
        let mut f0 = 1;
        let mut f1 = 1;
        let mut tmp = 1;
        for _ in 2..n+1{
            tmp = f0+f1;
            f0 = f1;
            f1 = tmp;
        }
        f1
    }
}

pub fn fib_recursive(n : u64) -> u64{
    if n <= 1 { 1 }
    else { fib_recursive(n-1) + fib_recursive(n-2) }
}

pub fn fib_recursive_with_cache_wrapper(n : u64) -> u64{
    if n == 0 { return 1; } //otherwise panics on avail[1]
    let n = n as usize; //this is probably fine
    let mut cache : Vec<u64> = vec![1; n+1];
    let mut avail : Vec<bool> = vec![false; n+1];
    avail[0] = true; avail[1] = true; //since all init to 1
    fib_recursive_with_cache(n,&mut cache,&mut avail)
}

fn fib_recursive_with_cache(n : usize, cache: &mut Vec<u64>,
                         avail: &mut Vec<bool>) -> u64{
    if avail[n] { cache[n] }
    else {
        let result = fib_recursive_with_cache(n-1, cache, avail) + &fib_recursive_with_cache(n-2, cache, avail);
        cache[n] = result;
        avail[n] = true;
        cache[n]
    }
}

pub fn fib_matrix(n : u64) -> u64{
    let mut fib = Matrix::new(2, 2);
    fib.array[0][1] = 1;
    fib.array[1][0] = 1;
    fib.array[1][1] = 1;
    fib = fib.pow(n);
    fib.array[1][1]
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn all_same_result_small(){
        ///Tests all functions for equivalence for a small n
        let r = fib_recursive(20);
        let d = fib_recursive_with_cache_wrapper(20);
        let i = fib_loop(20);
        let m = fib_matrix(20);
        assert_eq!(r, d);
        assert_eq!(r, i);
        assert_eq!(r, m);
        assert_eq!(d, i);
        assert_eq!(d, m);
        assert_eq!(i, m);
    }
}

