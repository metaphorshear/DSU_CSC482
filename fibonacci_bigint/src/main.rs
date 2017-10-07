extern crate hw3_fibonacci_challenge;
extern crate num;
use hw3_fibonacci_challenge::fibonacci::*;
use hw3_fibonacci_challenge::bench::*;
use num::bigint::BigUint;
use std::env;

fn bench_n(f: &Fn(u64)->BigUint, start: u64, end: u64){
    println!("#N\t\tTime taken (ns)");
    println!("#--------------------");
    for n in start..end {
        let t = timed_run_ns(f, n as u64);
        println!("{}\t\t{}", n, t);
    }
}

fn bench_low_n(f: &Fn(u64)->BigUint){
    bench_n(f, 0, 51);
}

fn bench_high_n(f: &Fn(u64)->BigUint){
    ///For benching all but the recursive version
    bench_n(f, 51, 20000);
}

fn bench_really_high_n(f: &Fn(u64)->BigUint){
    ///For benching the iterative and matrix versions
    bench_n(f, 20000, 100000);
}

fn bench(){

//    println!("r = [");
//    bench_low_n(&fib_recursive);
//    println!("];");
//
//    println!("d = [");
//    bench_low_n(&fib_recursive_with_cache_wrapper);
//    println!("];");
//
//    println!("i_big = [");
//    bench_low_n(&fib_loop);
//    println!("];");
//
//    println!("m_big = [");
//    bench_low_n(&fib_matrix);
//    println!("];");

    println!("d_big = [");
    bench_high_n(&fib_recursive_with_cache_wrapper);
    println!("];");

    println!("i_big = [");
    bench_high_n(&fib_loop);
    println!("];");

    println!("m_big = [");
    bench_high_n(&fib_matrix);
    println!("];");

    println!("i_huge = [");
    bench_really_high_n(&fib_loop);
    println!("];");

    println!("m_huge = [");
    bench_really_high_n(&fib_matrix);
    println!("];");

}


fn pp_bench(){
   println!("Times for low values of n");
   println!("=========================");
   println!("Recursive Algorithm\n");
   bench_low_n(&fib_recursive);
   println!("Memoized Recursive Algorithm\n");
   bench_low_n(&fib_recursive_with_cache_wrapper);
   println!("Iterative Algorithm\n");
   bench_low_n(&fib_loop);
   println!("Matrix multiplication Algorithm\n");
   bench_low_n(&fib_matrix);

   println!("Times for large values of n");
   println!("=========================");
   println!("Memoized Recursive Algorithm\n");
   bench_high_n(&fib_recursive_with_cache_wrapper);
   println!("Iterative Algorithm\n");
   bench_high_n(&fib_loop);
   println!("Matrix multiplication Algorithm\n");
   bench_high_n(&fib_matrix);
}

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() < 3 {
       if args.len() == 2 && &args[1] == "-b" {
            bench();
            return;
       }
       println!("\nUsage: {} <algorithm flag> <n>", &args[0]);
       println!("Prints the nth fibonacci number, where fib(0)=fib(1)=1");
       println!("\nPossible algorithm flags:\n");
       println!("-r\tRecursive algorithm");
       println!("-d\tMemoized Recursive algorithm");
       println!("-i\tIterative algorithm");
       println!("-m\tMatrix multiplication algorithm");
    }
   else{
       let algo = &args[1];
       let n = &args[2];
       let num = match n.parse::<u64>(){
           Err(why) => panic!("{:?}", why),
           Ok(n) => n, 
       };
       let result = match algo.as_ref(){
           "-r" => fib_recursive(num),
           "-d" => fib_recursive_with_cache_wrapper(num),
           "-i" => fib_loop(num),
           "-m" => fib_matrix(num),
            _   => fib_matrix(num)
       };
       println!("{}", result);

   }
}
