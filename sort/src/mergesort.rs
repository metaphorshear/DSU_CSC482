struct SortData<'a> {
    comps    : usize,
    v        : &'a mut Vec<i32>,
    aux      : Vec<i32>,
}

impl<'a> SortData<'a> {
    fn new(v : &'a mut Vec<i32>) -> SortData {
        let a = v.clone();
        SortData{ comps: 0, v: v, aux: a}
    }
    fn sort(&mut self, lo: usize, hi: usize){
        //lo and hi are indices that define an inclusive range
        //working with indices and a void function turned out a lot less confusing than slices
        if lo < hi {
            // if len is even, then two equal halves, otherwise len(mid-lo)=len(hi-(mid+1))+1
            let mid = lo + (hi - lo)/2; 
            self.sort(lo, mid);
            self.sort(mid+1, hi);
            self.merge(lo, mid, hi);
        }
    }
    fn merge(&mut self, lo: usize, mid: usize, hi: usize){
        {
            //copy values from v into aux over the relevant range
            let mut tmp = self.aux.get_mut(lo..(hi+1)).unwrap(); //"inclusive range syntax is experimental"
            let tm = {tmp.len()/2};
            let (tmpl, tmpr) = tmp.split_at_mut(tm); //necessary so the borrow system knows these slices don't overlap
            tmpl.copy_from_slice(self.v.get(lo..(mid+1)).unwrap()); 
            tmpr.copy_from_slice(self.v.get((mid+1)..(hi+1)).unwrap());
        } //now tmp is out of scope
        //let a = self.aux.get(lo..(hi+1)).unwrap(); //so I can borrow again
        let a = &self.aux[..];
        let (mut i, mut j) = (lo, mid+1);
        for k in lo..hi+1{
            if i > mid {self.v[k] = a[j]; j+=1}//left half exhausted
            else if j > hi {self.v[k] = a[i]; i+=1}//right half exhausted
            else if a[j] < a[i] {self.v[k] = a[j]; self.comps+=1; j+=1}
            else { self.v[k] = a[i]; i+=1; self.comps+=1;}
        }
    }
}
pub fn merge_sort(v: &mut Vec<i32>) -> usize{
    let size = v.len();
    let mut data = SortData::new(v);
    data.sort(0, size-1);
    data.comps
}