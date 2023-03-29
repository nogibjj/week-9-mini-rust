use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let mut numbers = vec![];

    for _ in 0..100 {
        let num = rng.gen_range(0..1000);
        numbers.push(num);
    }

    println!("{:?}", numbers);
}
