use rand::distributions::{Distribution, Uniform}; // 0.6.5

fn main() {
    let step = Uniform::new(0, 50);
    let mut rng = rand::thread_rng();
    let choice = step.sample(&mut rng);
    println!("{}", choice);
}