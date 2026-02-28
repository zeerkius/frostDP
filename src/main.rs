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
    let c = STB::new(6);
    let res =  c.goal();
    
    println!(" values {:?}",res);
    
    
    
    
}
