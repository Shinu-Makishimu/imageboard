use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, AccountId, env, log, Balance, PromiseOrValue, BorshStorageKey, ONE_YOCTO};

const POINT_ONE: Balance = 10000000000000000000000;

mod token_receiver;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Threads,
    OwnerId,
    ModersId,
    BansId,
    FTDeposits,
    Answers,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Thread {
    pub author: AccountId,
    pub premium: bool,
    pub text: String,
    pub is_closed: bool,
    pub answers: UnorderedMap<i32, String>,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ImageBoard {
    threads: UnorderedMap<i32, Thread>,
    owner: AccountId,
    moderators: Vector<AccountId>,
    threads_count: i32,
    bans: Vector<AccountId>,
    balance: Balance,
    ft_deposits: LookupMap<AccountId, Balance>,

}

impl Default for ImageBoard{
    fn default() -> Self {
        let owner = env::predecessor_account_id();


        Self { 
            threads: UnorderedMap::new(StorageKey::Threads), 
            owner, 
            moderators: Vector::new(StorageKey::ModersId),
            threads_count: 0,
            bans: Vector::new(StorageKey::BansId),
            balance: 0u128,
            ft_deposits: LookupMap::new(StorageKey::FTDeposits),
        
        }
    }
}



#[near_bindgen]
impl ImageBoard{

    #[init]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self { 
            threads: UnorderedMap::new(StorageKey::Threads), 
            owner, 
            moderators: Vector::new(StorageKey::ModersId),
            threads_count: 0,
            bans: Vector::new(StorageKey::BansId),
            balance: 0u128,
            ft_deposits: LookupMap::new(StorageKey::FTDeposits),

        }
    }

    pub fn assert_owner(&self) {
        assert_eq!(
            &self.owner,
            &env::predecessor_account_id(),
            "Not owner"
        );
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }
    #[payable]
    pub fn add_thread(&mut self, text: String) {
        // Add thread to the Unordered Map.  Imageboard can have only 500 threads. When the number of threads is more than 500, the first thread will be delete.
        // is_closed: bool. false -default. if thrue - thread is closed. only owner or moderators can close, or after 500 answers
        // text: string. text sent by the author
        // author: AccountId. who open thread.
        // answers: UnorderedMap with key: number of answers and message:string. When the number of answers is more than 500, thread will be closed (is_closed = thrue) 
        // premium: if author attached some deposit, the thread is considered premium. Passcode analogue.
        // adding pictures in development
        let premium = env::attached_deposit() >= POINT_ONE;
        let is_closed: bool = false;
        let author = env::predecessor_account_id();

        if self.threads.len() > 500 {
            let key: i32 = self.threads_count - 500;
            self.remove_thread(&key);
        }
        
        if self.is_banned(&author) {
            log!("access denied, reason - ban");
        }else {
            let answers: UnorderedMap<i32, String> = UnorderedMap::new(StorageKey::Answers);
            self.threads_count += 1;
            if self.threads_count == 3 { //% 10 == 0 {
                self.pay_ft(&author);
            }


            let message = Thread{
                        author, 
                        premium,
                        text, 
                        is_closed,
                        answers,
                    };
            
            self.threads.insert(&self.threads_count, &message);
           
            log!("tread add success");
        }
        
    }

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #[payable]
    pub fn pay_ft(&mut self, author: &AccountId) {


        if !self.ft_deposits.contains_key(&author)  {
            self.register_account(&author);
        }

        //how much we pay for  
        let deposit = ONE_YOCTO;



        let mut cur_bal = self.ft_deposits.get(&author).unwrap_or(0);
        cur_bal += deposit;
        self.balance -= deposit;
        self.ft_deposits.insert(&author, &cur_bal);

        //get the account ID to pay for storage for
        /*let storage_account_id = account_id 
            //convert the valid account ID into an account ID
            .map(|a| a.into())
            //if we didn't specify an account ID, we simply use the caller of the function
            .unwrap_or_else(env::predecessor_account_id);

        //get the deposit value which is how much the user wants to add to their storage
        let deposit = env::attached_deposit();

        //make sure the deposit is greater than or equal to the minimum storage for a sale
        assert!(
            deposit >= STORAGE_PER_SALE,
            "Requires minimum deposit of {}",
            STORAGE_PER_SALE
        );

        //get the balance of the account (if the account isn't in the map we default to a balance of 0)
        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        //add the deposit to their balance
        balance += deposit;

        let mut cur_bal = self.ft_deposits.get(&signer_id).unwrap_or(0);
        cur_bal += amount.0;
        self.ft_deposits.insert(&signer_id, &cur_bal);

        //insert the balance back into the map for that account ID
        self.storage_deposits.insert(&storage_account_id, &balance);
                let caller = env::predecessor_account_id();
        let cur_bal = self.ft_deposits.get(&caller).unwrap_or(0);
        require!(cur_bal >= amount.0, "Insufficient balance");

        // Subtract the amount from the caller's balance
        let new_bal:u128 = cur_bal - amount.0;

        self.ft_deposits.insert(&caller, &new_bal);

        
        
        */
    }
        


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////


    pub fn get_count(&self) -> i32 {
        self.threads_count
    }

    pub fn get_balance(&self) -> u128 {
        self.balance.clone()
    }
    
    pub fn get_threads(&self) -> Vec<(i32, String, String, bool)> {
        let mut b: Vec<(i32, String, String, bool)> =  vec![];
        for element in self.threads.to_vec(){
            b.push((element.0, element.1.author.to_string(), element.1.text, element.1.premium))
        }
        b
    }

    pub fn get_the_thread(&self, number: i32) -> String {
        //self.threads.get(&number).unwrap().text.clone()
        match self.threads.get(&number) {
            Some(thread) => thread.text.clone(),
            None => "thread not found".to_string(),
        }
    }

    pub fn remove_thread(&mut self, key: &i32) {
        let author = env::predecessor_account_id();  

        if (self.is_moder(&author) ) | (self.owner.to_string() == author.to_string())  {
            match self.threads.remove(&key) {
                Some(_result) => { log!("Removing thread {:?} succes", key);},
                None => { log!("Removing thread {:?} failed", key); },
        }
    
        } else {
            log!("permission denied");
        }

    }

    pub fn ban_thread(&mut self, number: i32) {
        let author = env::predecessor_account_id();  

        if (self.is_moder(&author) ) | (&self.owner.to_string() == &author.to_string())  {
            let mut thread = self.threads.get(&number).unwrap();
            thread.is_closed = true;
            self.threads.insert(&number, &thread);
            log!("thred is banned");
        } else {
            log!("thread ban failed");
        }

    }


    pub fn add_moder(&mut self, user_id: AccountId) -> String{
        let call_account = env::predecessor_account_id();
        log!("{:?}",call_account);
        if call_account.to_string() == self.owner.to_string() {
            self.moderators.push(&user_id);
            "success".to_string()
        } else {
            "denied".to_string()
        }

    }

    pub fn is_moder(&self, name: &AccountId) -> bool {
        if self.moderators.iter().any(|x| x.to_string() == name.to_string()) {
            true
        } else {
            false
        }
        
    }

    pub fn get_moders(&self) -> Vec<String> {
        self.moderators.iter().map(|x| x.to_string()).collect()
    }

    pub fn delete_moder(&mut self, user_id:AccountId) -> String {
        let call_account = env::predecessor_account_id();
        if call_account.to_string() == self.owner.to_string() {
            let index = self.moderators
            .iter()
            .position(|x| x.as_str() == user_id.as_str())
            .unwrap();
            
            self.moderators.swap_remove(index as u64);
            "success".to_string()
        } else {
            "denied".to_string()
        }

    }

    pub fn add_answers(&mut self, thread_number: i32, text: String) -> String {
        //function to add a reply to a thread.
        //To add an answer, you need a key (thread number).
        //There are two checks: the thread is closed and account is banned.
        let mut thread: Thread =  self.threads.get(&thread_number).unwrap();
        let author: AccountId = env::predecessor_account_id();  //?? should i use signer_account_id insted?

        if thread.is_closed {
           "thread is closed".to_string() 
        } else if self.is_banned(&author) {
            "banned".to_string() 
        } else {
            let mut count = thread.answers.len() as i32;

            match count {
                0 => {
                    log!("zero calls");
                    thread.answers.insert(&count, &text); 
                    self.threads.insert(&thread_number, &thread);
                    "first post".to_string()
                },
                500 => {
                    thread.is_closed = true;
                    self.threads.insert(&thread_number, &thread);
                    "thread is closed".to_string()
                },
                _ => {
                    log!("normal call");
                    count += 1;
                    thread.answers.insert(&count, &text); 
                    self.threads.insert(&thread_number, &thread);
                    "succes".to_string()
                },
            
            }
        }
    }


    pub fn get_thread_answers (&self, thread_number: i32) -> String {
        let thread: Thread = self.threads.get(&thread_number).unwrap();

        thread.answers.
            values_as_vector().
            to_vec().
            into_iter().
            map(|mut x| {x.push_str("\n"); x})
            .collect()
    }


    pub fn ban(&mut self, user: &AccountId) {
        let author = env::predecessor_account_id();  
        if (self.is_moder(&author) ) | (&self.owner.to_string() == &author.to_string())  {
            self.bans.push(&user);
            log!("ban");

        } else {
            log!("ban fail");

        }

    }

    pub fn is_banned(&self, name: &AccountId) -> bool {
        if self.bans.iter().any(|x| x.to_string() == name.to_string()) {
            true
        } else {
            false
        }
    }

    pub fn get_bans(&self) -> Vec<String> {
        self.bans.iter().map(|x| x.to_string()).collect()
    
    }


    pub fn remove_ban (&mut self, user: AccountId) {
        let author = env::predecessor_account_id();  
        if (self.is_moder(&author) ) | (&self.owner.to_string() == &author.to_string())  {
            let index: usize = self.bans
                .iter()
                .position(|x| x.to_string() == user.to_string())
                .unwrap();
            self.bans.swap_remove(index as u64);
            log!("unban success");
        } else {
            log!("unban fail");
        }
    }

    #[private]
    #[result_serializer(borsh)]
    pub fn finish_deposit(
        // self balance will be changed after receiver call.
        &mut self,
        #[serializer(borsh)]account_id: AccountId,
        #[serializer(borsh)]amount: U128,
        #[serializer(borsh)]msg: String,
    ) -> PromiseOrValue<U128> {
        log!("account {:?}, message {:?},", account_id, msg);
        self.balance += amount.0;
        PromiseOrValue::Value(U128(0))
    }

    pub fn storage_deposits(&self, account_id: AccountId) -> U128 {
        self.ft_deposits.get(&account_id).unwrap_or(0).into()
    }



    fn register_account(&mut self, account_id: &AccountId) {
        if self.ft_deposits.insert(account_id, &0).is_some() {
            env::panic_str("The account is already registered");
        }
    }

    
}


#[cfg(test)]
mod tests {
    use near_sdk::log;

    use super::*;
    #[test]
    fn create_board() {
        let contract: ImageBoard = ImageBoard::default();
        let owner:AccountId = contract.get_owner();
        log!("owner {:?}", owner.to_string());
    
    }

    #[test]
    fn add_thread() {
        let mut contract: ImageBoard = ImageBoard::default();

        contract.add_thread("sup NEARach. There is some tyan...".to_string());
        
        assert_eq!(1, contract.get_threads().len());


        for _ in 1..50 {
            contract.add_thread("threads dudos".to_string());
          

        }
        log!("total threads {:?}", contract.get_threads().len()); 
        log!("count {:?}", contract.get_count());   

        assert_eq!(50, contract.get_threads().len());

        let thread: String = contract.get_the_thread(4);

        log!("thread numb 4 for check  = {:?}", thread);
        assert_eq!(thread, "threads dudos".to_string());
        
        
        let thread: String = contract.get_the_thread(420);
        assert_eq!(thread, "thread not found".to_string());

        contract.remove_thread(&1);
        contract.remove_thread(&1);
        assert_eq!(49, contract.get_threads().len());

    }

    #[test]
    fn add_get_answers() {
        let mut contract: ImageBoard = ImageBoard::default();

        contract.add_thread("sup NEARach. There is some tyan...".to_string());
        
        
        let answ: String = contract.add_answers(1, "bbs or go".to_string());
        log!("status  = {:?}", answ);
        contract.add_answers(1, "gurls wthout magic stick do not exist".to_string());
        contract.add_answers(1, "show her magic stick!".to_string());
        let thread: String = contract.get_the_thread(1);

        log!("thread numb 1 for check  = {:?}", thread);
        
        
        let answ: String = contract.get_thread_answers(1);
        for i in answ.lines() {
            log!("answers  = {:?}", i);
        }
    }

    #[test]
    fn moders() {
        let sarina: AccountId = "sarina.near".parse().unwrap();
        let mars: AccountId = "mars.near".parse().unwrap();
        let bailey: AccountId = "bailey.near".parse().unwrap();

        let mut contract: ImageBoard = ImageBoard::default();
        log!("owner  {:?}", contract.get_owner());
        
        let moder: String = contract.add_moder(mars);
        log!("moder {:?}", moder);

        let check: bool = contract.is_moder(&sarina);
        log!("check  {:?}", check);

        let list_moder: Vec<String> = contract.get_moders();
        log!("moder list  {:?}", list_moder);
        
        let _ban = contract.ban(&bailey);
        let ban_list = contract.get_bans();
        log!("ban list  {:?}", ban_list);


    }

    #[test]
    fn balance() {
        let contract: ImageBoard = ImageBoard::default();
        let balanse: u128 = contract.get_balance();
        log!("balanse {:?}", balanse);
    
    }
        


}
