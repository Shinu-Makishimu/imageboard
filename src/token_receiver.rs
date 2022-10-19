use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{Gas, AccountId};
use near_sdk::json_types::U128;




use crate::*;



const FT_FINISH_DEPOSIT_GAS: Gas = Gas(Gas::ONE_TERA.0 * 10);



#[near_bindgen]
impl FungibleTokenReceiver for ImageBoard {

    fn ft_on_transfer(&mut self, sender_id:AccountId , amount: U128, msg: String,) -> PromiseOrValue<U128> {
        assert_eq!(env::predecessor_account_id(), sender_id );
        self.balance += amount;
        
        PromiseOrValue::Value(U128(0))
        
    }
}