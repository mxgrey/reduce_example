
struct Reducer<'a> {
    original: &'a Vec<u8>,
    data: Vec<u8>,
}

impl<'a> Reducer<'a> {
    fn reduce(&self) -> Reducer {
        let mut new_data = self.data.clone();
        new_data.pop();
        return Reducer {
            original: self.original,
            data: new_data,
        };
    }
}

fn main() {

    let original_data: Vec<u8> = (0..10).collect();

    let mut r = Reducer {
        original: &original_data,
        data: original_data.clone(),
    };

    for _ in 0..5 {
        println!("Values: {:?}", r.data);
        r = r.reduce();
    }
}
