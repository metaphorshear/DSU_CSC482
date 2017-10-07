//import functions from sort files
extern crate sort;
use sort::bubblesort::bubble_sort;
use sort::mergesort::merge_sort;
//needed to grab command-line args
use std::env;
//needed for file i/o
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <sorttype> <filename>", &args[0]);
        println!("Where sorttype is -b for bubble or -m for mergesort");
    }
    else {
        let sorttype = &args[1];
        let filename = &args[2];
        let f = File::open(filename).expect("Error reading file!");
        let reader = BufReader::new(f);
        let mut vector : Vec<i32> = vec![];
        for line in reader.lines() {
            //reader.lines() is an iterator
            //each line in reader.lines is an io::Result<String>
            //the Result will be either Ok or Err (not unlike the Maybe monad)
            match line {
                Err(why) => panic!("{:?}", why),
                Ok(string) => match string.trim().parse::<i32>(){
                    //not sure how necessary this match is
                    //but I guess unwrap is frowned upon
                    Err(why)    => panic!("{:?}", why),
                    Ok(number)=> vector.push(number)
                }
            }
        }
        //println!("Vector read!  Size: {}", vector.len());
        //println!("Input values: {:?}", vector);
        let result = match sorttype.as_ref() {
            "-b" => bubble_sort(&mut vector),
            "-m" => merge_sort(&mut vector),
            _    => merge_sort(&mut vector)
        };
        for v in vector{
            println!("{}", v);
        }
        eprintln!("{}", result); //these are total comparisons; I removed the thing that said that
        //so it would be easier to dump numbers in a file and plot them
        //println!("Sorted values: {:?}", vector);
    }
}
