// By Lingba Saner 24213 🍥 v0.11-dev
use std::env;
mod rxcalc;
use rxcalc::calc;
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", calc(&args[1]));
    } else {
        println!("You need to pass in parameters like this: [executefile] '[computational expression]'");
    }
}