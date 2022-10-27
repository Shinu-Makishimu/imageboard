use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::AccountId;
use near_sdk::json_types::U128;




use super::*;





#[near_bindgen]
impl StorageManagement for ImageBoard {
    //#[allow(unused_variables)]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {

        let amount: Balance = env::attached_deposit();
        let account_id = account_id.unwrap_or_else(env::predecessor_account_id);

        if self.ft_deposits.contains_key(&account_id) {
            log!("The account is already registered, refunding the deposit");
            if amount > 0 {
                Promise::new(env::predecessor_account_id()).transfer(amount);
            }
        } else {
            let min_balance = self.storage_balance_bounds().min.0;
            if amount < min_balance {
                env::panic_str("The attached deposit is less than the minimum storage balance");
            }

            self.internal_register_account(&account_id);
            let refund = amount - min_balance;
            if refund > 0 {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
        }
        self.internal_storage_balance_of(&account_id).unwrap()

    }


    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        let required_storage_balance =
            Balance::from(self.account_storage_usage) * env::storage_byte_cost();
        StorageBalanceBounds {
            min: required_storage_balance.into(),
            max: Some(required_storage_balance.into()),
        }
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.internal_storage_balance_of(&account_id)
    }
}