
struct Reducer<'a> {
    original: &'a str,
    stop: usize,
}

impl<'a> Reducer<'a> {

    fn new<'b>(original: &'b str) -> Reducer<'b> {
        return Reducer{
            original: original,
            stop: original.len(),
        }
    }

    fn reduce(&self) -> Reducer {
        return Reducer {
            original: &(*self.original),
            stop: self.stop-1,
        };
    }

    fn print(&self) {
        for i in 0..self.stop {
            print!("{}", self.original.chars().nth(i).unwrap());
        }
        print!("\n");
    }
}

fn main() {

    let original = "Hello, world!";

    let mut r = Reducer::new(&original);

    for _ in 0..5 {
        r = r.reduce();
        r.print();
    }
}
