use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, AccountId, env, log, Balance, Promise, PromiseOrValue, ext_contract };

const POINT_ONE: Balance = 10000000000000000000000;

mod token_receiver;



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

}


#[ext_contract(ext_self)]
pub trait ExtContract {
    #[result_serializer(borsh)]
    fn finish_deposit(&mut self, account_id: AccountId, amount: U128, msg: String,) -> PromiseOrValue<U128>; 

    #[result_serializer(borsh)]
    fn finish_withdraw(&self,) ->Promise;

}


impl Default for ImageBoard{
    fn default() -> Self {
        let owner = env::predecessor_account_id();


        Self { 
            threads: UnorderedMap::new(b"threads".to_vec()), 
            owner, 
            moderators: Vector::new(b"moderators".to_vec()),
            threads_count: 0,
            bans: Vector::new(b"bans".to_vec()),
            balance: 0u128,
        
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
            threads: UnorderedMap::new(b"threads".to_vec()), 
            owner, 
            moderators: Vector::new(b"moderators".to_vec()),
            threads_count: 0,
            bans: Vector::new(b"bans".to_vec()),
            balance: 1u128,
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
        let premium = env::attached_deposit() >= POINT_ONE;
        let is_closed: bool = false;
        let author = env::predecessor_account_id();

        if self.threads.len() > 500 {
            let key: i32 = self.threads_count - 500;
            self.remove_thread(&key);

        }
        if self.bans.iter().any(|x| x.to_string() == author.to_string()){
            log!("access denied, reason - ban");
        }else {
            let answers: UnorderedMap<i32, String> = UnorderedMap::new(b"answers".to_vec());

            let message = Thread{
                        author, 
                        premium,
                        text, 
                        is_closed,
                        answers,
                    };
            self.threads_count += 1;
            
            self.threads.insert(&self.threads_count, &message);
            log!("tread add success");
        }
        
    }
        
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

        if (self.is_moder(&author) == *"moder") | (self.owner.to_string() == author.to_string())  {
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

        if (self.is_moder(&author) == "moder".to_string()) | (&self.owner.to_string() == &author.to_string())  {
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

    pub fn is_moder(&self, name: &AccountId) -> String {
        if self.moderators.iter().any(|x| x.to_string() == name.to_string()) {
            "moder".to_string()
        } else {
            "not_moder".to_string()
            
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
        let mut thread =  self.threads.get(&thread_number).unwrap();
        let author = env::predecessor_account_id();  //?? should i use signer_account_id insted?

        if thread.is_closed {
           "thread is closed".to_string() 
        
        } else if self.is_banned(&author) == "banned "{
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
        if (self.is_moder(&author) == "moder".to_string()) | (&self.owner.to_string() == &author.to_string())  {
            self.bans.push(&user);
            log!("ban");

        } else {
            log!("ban fail");

        }

    }

    pub fn is_banned(&self, name: &AccountId) -> String {
        if self.bans.iter().any(|x| x.to_string() == name.to_string()) {
            "banned".to_string()
        } else {
            "not_banned".to_string()
        }
    }

    pub fn get_bans(&self) -> Vec<String> {
        self.bans.iter().map(|x| x.to_string()).collect()
    
    }


    pub fn remove_ban (&mut self, user: AccountId) {
        let author = env::predecessor_account_id();  
        if (self.is_moder(&author) == "moder".to_string()) | (&self.owner.to_string() == &author.to_string())  {
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
        #[serializer(borsh)]amount: u128,
        #[serializer(borsh)]msg: String,
    ) -> PromiseOrValue<u128> {
        log!("account {}, message {},", account_id, msg);
        self.balance += amount;
        PromiseOrValue::Value(0)
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
        log!("count{:?}", contract.get_count());   

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

        let check: String = contract.is_moder(&sarina);
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
