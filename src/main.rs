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

fn main() {
    let c = CoinChange::new(vec![1,2,5],10);
    let res =  c.maxContribution();
    
    println!(" values {:?}",res);
    
    
    
    
}
