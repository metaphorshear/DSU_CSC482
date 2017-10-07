use matrix::Matrix;

use num::bigint::BigUint;
use num::CheckedAdd;

pub fn fib_loop(n: u64) -> BigUint{
    if n <= 1 { BigUint::from(1u8) }
    else {
        let mut f0 : BigUint = BigUint::from(1u8);
        let mut f1 = f0.clone();
        let mut tmp = f1.clone();
        for _ in 2..n+1{
            tmp = f0.checked_add(&f1).unwrap();
            f0 = f1;
            f1 = tmp;
        }
        f1
    }
}

pub fn fib_recursive(n : u64) -> BigUint{
    if n <= 1 { BigUint::from(1u8) }
    else { fib_recursive(n-1) + fib_recursive(n-2) }
}

pub fn fib_recursive_with_cache_wrapper(n : u64) -> BigUint{
    if n == 0 { return BigUint::from(1u8); } //otherwise panics on avail[1]
    let n = n as usize; //this is probably fine
    let mut cache : Vec<BigUint> = vec![BigUint::from(1u8); n+1];
    let mut avail : Vec<bool> = vec![false; n+1];
    avail[0] = true; avail[1] = true; //since all init to 1
    fib_recursive_with_cache(n,&mut cache,&mut avail)
}

fn fib_recursive_with_cache(n : usize, cache: &mut Vec<BigUint>,
                         avail: &mut Vec<bool>) -> BigUint{
    if avail[n] { cache[n].clone() }
    else {
        let result = fib_recursive_with_cache(n-1, cache, avail)
                            .checked_add(&fib_recursive_with_cache(n-2, cache, avail))
                            .unwrap();
        cache[n] = result;
        avail[n] = true;
        cache[n].clone()
    }
}

pub fn fib_matrix(n : u64) -> BigUint{
    let mut fib = Matrix::new(2, 2);
    fib.array[0][1] = BigUint::from(1u8);
    fib.array[1][0] = BigUint::from(1u8);
    fib.array[1][1] = BigUint::from(1u8);
    fib = fib.pow(n);
    fib.array[1][1].clone()
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
    #[test]
    fn most_same_result_large(){
        ///Tests the faster functions for equivalence with large n
        let d = fib_recursive_with_cache_wrapper(1000);
        let i = fib_loop(1000);
        let m = fib_matrix(1000);
        assert_eq!(d, i);
        assert_eq!(d, m);
        assert_eq!(i, m);
    }
    #[test] #[ignore]
    fn same_result_really_large(){
        ///Tests the iterative function vs the matrix function for really big n
        let i = fib_loop(100000);
        let m = fib_matrix(100000);
        assert_eq!(i, m);
    }
}

