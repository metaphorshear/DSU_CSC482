
pub fn bubble_sort(v : &mut Vec<i32>) -> usize {
    let mut n = v.len();
    let mut newn = 0;
    let mut comps = 0;
    while n>0 {
        newn = 0;
        for i in 1..n {
            let r = v[i-1] > v[i];
            comps += 1;
            if r {
                v.swap(i, i-1);
                newn = i;
            }
        }
        n = newn;
    }
    comps
}
