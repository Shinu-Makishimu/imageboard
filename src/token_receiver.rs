use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::AccountId;
use near_sdk::json_types::U128;

use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for ImageBoard {
    // this is function which impl receiver for imageboard contract. after predecessor is sender check, call finish deposit func. 
    fn ft_on_transfer(&mut self, sender_id:AccountId , amount: U128, msg: String,) -> PromiseOrValue<U128> {
        assert_eq!(env::signer_account_id(), sender_id);

        log!("amount {:?}", amount.0);

        self.finish_deposit(sender_id, amount, msg);
        PromiseOrValue::Value(U128(0))
    }
}