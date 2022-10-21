use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{Gas, AccountId};
use near_sdk::json_types::U128;




use crate::*;



const FT_FINISH_DEPOSIT_GAS: Gas = Gas(Gas::ONE_TERA.0 * 10);



#[near_bindgen]
impl FungibleTokenReceiver for ImageBoard {
    // this is function which impl receiver for imageboard contract. after predecessor is sender check, call finish deposit func. 
    fn ft_on_transfer(&mut self, sender_id:AccountId , amount: U128, msg: String,) -> PromiseOrValue<U128> {
        assert_eq!(env::predecessor_account_id(), sender_id);
        ext_self::ext(env::current_account_id())
            .with_static_gas(FT_FINISH_DEPOSIT_GAS)
            .finish_deposit(env::predecessor_account_id(), amount, msg);
        PromiseOrValue::Value(U128(0))
        
    }
}