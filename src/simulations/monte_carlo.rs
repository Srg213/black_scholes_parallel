use rand::Rng;
use obj::Option;
use rand_distr::{Normal, Distribution};

struct Simulation{
    time_step: u64,
    option: Option,
}

impl Simulation {

    pub fn new(time_step: u64, option: &Option){
        Self{
            time_step,
            option: option.clone(),
        }
    }

    pub fn euler_maruyama_method(&self) -> u64 {
        let drift = 0;
        let norm = Normal::new(0.0, 1.0).unwrap();
        let final_price = self.option.spot_price * (( drift - self.option.volatility.pow(2)/2 ) * self.time_step + self.option.volatility * self.time_step.sqrt()* norm).exp();
    }
}


fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random number between 0 and 10
    let random_number: i32 = rng.gen_range(0..10);
    println!("Random number between 0 and 10: {}", random_number);

    // Generate a random floating-point number between 0 and 1
    let random_float: f64 = rng.gen();
    println!("Random floating-point number between 0 and 1: {}", random_float);

    // Generate a random boolean
    let random_bool: bool = rng.gen();
    println!("Random boolean: {}", random_bool);
}