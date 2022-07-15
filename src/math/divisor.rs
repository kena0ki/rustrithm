#[derive(Debug,Clone)]
pub struct Divisor {
    pub n: usize,
    pub divisors: Vec<Vec<usize>>,
}

impl Divisor {
    /// Initializes Divisor.
    /// O(NlogN)
    pub fn new(n:usize) -> Self{
        let mut divisors = vec![Vec::new();n+1];
        for i in 1..n+1 {
            for j in 1..n/i+1 {
                divisors[j*i].push(i);
            }
        }
        return Self {n, divisors};
    }
    pub fn divisors(&self, x:usize) -> &Vec<usize> {
        return &self.divisors[x];
    }
}
