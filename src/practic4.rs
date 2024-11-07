
pub struct Romb {
    size: i32,
}

impl Romb{
    pub fn new(size: i32) -> Romb{
        Romb{ size }
    }

    pub fn rombBuilder(self){
        let (mut point_start, mut point_end)  = ((self.size /2), (self.size /2));

        for i in 0..self.size{
            for j in 0..self.size{

                let symbol = if(j >= point_start && j <= point_end) { "*" } else { " " };
                print!("{symbol}");

            }

            if(i < (self.size / 2)) {
                point_end   += 1;
                point_start -= 1;
            } else {
                point_start += 1;
                point_end   -= 1;
            }
            println!()
        }
    }
}

#[test]
fn main() {
    let romb = Romb::new(11);
    romb.rombBuilder();
}
