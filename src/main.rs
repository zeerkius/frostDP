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
use::std::*;
use frost::CoinChange;
use frost::STB;


fn main() {
    let sim = STB::new(6,2).unwrap();
    
    let d = vec![0.0,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1.0];
    
    
    let b : i32 = 10;
    let e : u32 =  2;
    let k : i32 = b.pow(e);
    
    
    
    println!(" {:?}",sim.value_iteration(0.5,k));
    
    


    
}
