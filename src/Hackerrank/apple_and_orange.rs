pub struct Counter{
    s: i32,
    t: i32,
    a: i32,
    b: i32,
}

impl Counter{
    pub fn new(s:i32,t:i32,a:i32,b:i32) -> Self{
        Self{s,t,a,b}
    }

    fn check(&self, fruits: &[i32], tree_pos: i32) -> i32{
        let mut count = 0;
        for &fruit in fruits{
            let pos = tree_pos + fruit;
            if pos >= self.s && pos <= self.t {
                count += 1;
            }
        }
        count
    }

    pub fn get_result(&self, apples: &[i32], oranges: &[i32]) {
        let count_apples = self.check(apples, self.a);
        let count_orange = self.check(oranges, self.b);

        println!("{}\n{}", count_apples, count_orange);
    }
}

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let fruitCounter = Counter::new(s,t,a,b);
    fruitCounter.get_result(&apples, &oranges);
}

#[test]
fn main() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    let fruitCounter = Counter::new(s,t,a,b);
    fruitCounter.get_result(&apples, &oranges);
}
