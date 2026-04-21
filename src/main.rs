#![allow(
    unused_variables,
    dead_code,
    non_snake_case,
    unused_parens,
    unused_variables,
    non_snake_case,
    unreachable_code,
    unused_imports,
    unused_assignments,
    unused_mut
)]


mod frost;
use frost::CoinChange;
use frost::STB;


fn main() {
    let sim = STB::new(6,2).unwrap();
    
    let boundary = vec![0.1,0.15,0.2,0.25,0.3,0.35,0.4,0.5,0.505,0.6,0.65,0.7,0.8,0.9,1.0];
    
    println!(" {:?}",sim.stb_test(boundary));
    
    


    
}
