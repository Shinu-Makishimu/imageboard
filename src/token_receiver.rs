use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {

        ext_self::ext(env::current_account_id())
            .with_static_gas(FT_FINISH_DEPOSIT_GAS)
            .finish_deposit(env::predecessor_account_id(), amount.0, eth_address);
        PromiseOrValue::Value(U128(0))
    }
}