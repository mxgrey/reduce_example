
struct Reducer {
    // original: &'a str,
    stop: usize,
}

impl Reducer {

    fn new(original: & str) -> Reducer {
        return Reducer{
            //original: original,
            stop: original.len(),
        }
    }

    fn reduce(&self) -> Reducer {
        return Reducer {
            //original: self.original,
            stop: self.stop-1,
        };
    }

    fn print(&self, original: &str) {
        for i in 0..self.stop {
            print!("{}", original.chars().nth(i).unwrap());
        }
        print!("\n");
    }
}

fn main() {

    let original = "Hello, world!";

    let mut r = Reducer::new(&original);

    for _ in 0..5 {
        r = r.reduce();
        r.print(original);
    }
}
