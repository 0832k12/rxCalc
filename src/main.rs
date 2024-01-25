// By Lingba Saner 24125 🍥 v0.9-final
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