#[derive(Debug)]
struct Bucket {
    liters: u32,
}

impl Bucket {
    fn pour(&mut self, amount: u32) {
        self.liters -= amount;
    }

    fn fill(&mut self, amount: u32) {
        self.liters += amount;
    } 
}

fn main() {
    let mut source_bucket = Bucket {liters: 20};
    let mut target_bucket = Bucket {liters: 10};

    source_bucket.fill(3);
    target_bucket.pour(3);
    println!("Source Bucket: {:?}", source_bucket);
    println!("Target Bucket: {:?}", target_bucket); 
}
