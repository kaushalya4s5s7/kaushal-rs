use crate::admin::{read_administrator,write_administrator,has_administrator};
use crate::allowance::{read_allowance,write_allowance,spend_allowance};
use crate:: balance::{read_balance,write_balance,receive_balance,spend_balance};
use crate:: metadata::{read_decimal,read_name,read_symbol,write_metadata};
use crate:: storage_types::{INSTANCE_BUMP_AMOUNT,INSTANCE_LIFETIME, State,DataKey};
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract,contractimpl,Env,Address,String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

pub fn check_negative(amount:i128){
 if amount< 0{
    panic!("negative value is not allowed");
 };
}

#[contract]
pub struct Token;


#[contractimpl]
impl Token{
   
   pub fn initialize(e:&Env,admin:Address, decimal:u32, name:String, symbol:String){
     if has_administrator(&e){
        panic!("already intialised")
     }
     write_administrator(&e, &admin);
     if decimal > u8::MAX.into(){
        panic!("Decimal must fit in u8");
     }
     write_metadata(
        &e,
        TokenMetadata{ decimal,name,symbol}
           
        
     );
   }

   pub fn mint(e:&Env , to:Address , amount:i128){
    check_negative(amount);
    let admin = read_administrator(&e);
    admin.require_auth();

    e.storage().instance().extend_ttl(INSTANCE_LIFETIME, INSTANCE_BUMP_AMOUNT);

   }

   pub fn setadmin(e:&Env,new_admin:Address){
    let admin = read_administrator(&e);
    admin.require_auth();
    e.storage().instance().extend_ttl(INSTANCE_LIFETIME,INSTANCE_BUMP_AMOUNT);
    write_administrator(&e, &new_admin);
   }

  pub fn set_state(e: &Env, addr: Address, state: State) {
    let key = DataKey::State(addr.clone());
    e.storage().instance().set(&key, &state);
}

   pub fn get_state(e: &Env, addr: Address) -> State {
    let key = DataKey::State(addr.clone());
    e.storage()
        .instance()
        .get::<_, State>(&key)
        .unwrap_or(State::Unfreeze)
}

  pub fn restricted_action(e: &Env, caller: Address) {
    match Self::get_state(e, caller) {
        State::Freeze => panic!("Access denied: inactive user"),
        State::Unfreeze => {
            // Proceed
        }
    }
}




   
}

#[contractimpl]
impl token:: Interface for Token{
    fn allowance(e:Env,from : Address, spender:Address)->i128{
      e.storage().instance().extend_ttl(INSTANCE_LIFETIME, INSTANCE_BUMP_AMOUNT);
    
    read_allowance(&e,from ,spender).amount
    }

    fn approve(e:Env , from : Address , spender:Address,amount: i128, expiration_ledger: u32){
        from.require_auth();
        check_negative(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
        .events()
        .approve(from,spender,amount,expiration_ledger)
    }

    fn balance(e:Env, id:Address)->i128{
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME,INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }
    fn transfer(e:Env, from:Address,to :Address,amount:i128){
        Token::restricted_action(&e, from.clone());
        from.require_auth();
        check_negative(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME,INSTANCE_BUMP_AMOUNT);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
          TokenUtils::new(&e)
        .events()
        .transfer(from,to,amount);
   
    }
     fn transfer_from(e:Env,spender:Address, from:Address,to :Address,amount:i128){
        Token::restricted_action(&e, spender.clone());

        spender.require_auth();
        check_negative(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME,INSTANCE_BUMP_AMOUNT);
        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
          TokenUtils::new(&e)
        .events()
        .transfer(from,to,amount);
            e.storage().instance().extend_ttl(INSTANCE_LIFETIME,INSTANCE_BUMP_AMOUNT);

   
    }

    fn burn(e:Env, from: Address, amount:i128){
        from.require_auth();
        check_negative(amount);
                e.storage().instance().extend_ttl(INSTANCE_LIFETIME,INSTANCE_BUMP_AMOUNT);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from,amount);
    }
    fn burn_from(e:Env, spender:Address,from: Address, amount:i128){
        spender.require_auth();
        check_negative(amount);
                e.storage().instance().extend_ttl(INSTANCE_LIFETIME,INSTANCE_BUMP_AMOUNT);
                spend_allowance(&e,from.clone(),spender, amount);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from,amount);
    }
   
    


   fn decimals(e:Env)->u32{
     read_decimal(&e)
   }
   
   fn name(e:Env)->String{
    read_name(&e)
   }

   fn symbol(e:Env)->String{
    read_symbol(&e)
   }


}