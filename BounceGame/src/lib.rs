#![no_std]
use soroban_sdk::{Symbol,symbol_short,contract,contractimpl,Env,log};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Incrementor;

#[contractimpl]
impl Incrementor{
     pub fn increment(env:Env)-> u32{
         let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
         log!(&env,"count:{}" count);
         count+=1;
         env.storage().instance().set(&COUNTER,&count);
         count

     }
}

