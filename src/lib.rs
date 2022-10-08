use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::{near_bindgen, AccountId, env, log};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Thread {
    pub author: AccountId,
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

}


impl Default for ImageBoard{
    fn default() -> Self {
        let owner = env::predecessor_account_id();
        Self { 
            threads: UnorderedMap::new(b"threads".to_vec()), 
            owner, 
            moderators: Vector::new(b"moderators".to_vec()),
            threads_count: 0,
        
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
        }
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn add_thread(&mut self, text: String) {
        let is_closed: bool = false;
        let author = env::predecessor_account_id();  //?? should i use signer_account_id insted?

        if self.threads.len() > 500 {
            let key: i32 = self.threads_count - 500;
            self.remove_thread(&key);

        }

        let answers: UnorderedMap<i32, String> = UnorderedMap::new(b"answers".to_vec());

        let message = Thread{
                    author, 
                    text, 
                    is_closed,
                    answers,
                };
        self.threads_count += 1;
        
        self.threads.insert(&self.threads_count, &message);
        
    }
        
    pub fn get_count(&self) -> i32 {
        self.threads_count
    }
    
    #[result_serializer(borsh)]
    pub fn get_threads(&self) -> Vec<(i32, Thread)> {
        self.threads.to_vec()     
    }

    pub fn get_the_thread(&self, number: i32) -> String {
        self.threads.get(&number).unwrap().text.clone()
    }

    pub fn remove_thread(&mut self, key: &i32) {
        self.threads.remove(&key);

    }

    pub fn ban_thread(&self, number: i32) {
        let mut thread = self.threads.get(&number).unwrap();
        thread.is_closed = true

    }


    pub fn add_moder(&mut self, user_id: AccountId){
        self.moderators.push(&user_id);

    }

    pub fn delete_moder(&mut self, user_id:AccountId) {
        let index = self.moderators
            .iter()
            .position(|x| x.as_str() == user_id.as_str())
            .unwrap();
            
        self.moderators.swap_remove(index as u64);

    }

    pub fn add_answers(&mut self, thread_number: i32, text: String) -> String {
        let mut thread =  self.threads.get(&thread_number).unwrap();
        log!("answ cont {:?}", thread.answers.len());
        if thread.is_closed {
            "thread is closed".to_string()
        } else {
            let mut count = thread.answers.len() as i32;

            match count {
                0 => {
                    log!("zero calls");
                    thread.answers.insert(&count, &text); 
                    "first post".to_string()


                },
                500 => {
                    thread.is_closed = true;
                    "thread is closed".to_string()

                },
                _ => {
                    log!("normal call");
                    count += 1;
                    thread.answers.insert(&count, &text); 
                    "succes".to_string()

                },
            }
        }
    }

    pub fn get_thread_answers(&self, thread_number: i32) -> Vec<String> {
        let thread =  self.threads.get(&thread_number).unwrap();
        thread.answers.values_as_vector().to_vec()
    }
}

/*
#[cfg(test)]
mod tests {
    use near_sdk::log;

    use super::*;
    
    #[test]
    fn add_thread() {
        let mut contract = ImageBoard::default();
        for _ in 1..100 {
            contract.add_thread("there are all dead".to_string());
          

        }
        log!("total threads {:?}", contract.get_threads().len()); 
        log!("count{:?}", contract.get_count());   

        assert_eq!(99, contract.get_threads().len());

        let thread = contract.get_the_thread(4);

        log!("thread numb 4 for check  = {:?}", thread);
        assert_eq!(thread, "there are all dead".to_string());
        log!("add first answer");
        contract.add_answers(4, "PTN PNX".to_string());
        log!("get answ  = {:?}", contract.get_thread_answers(4));
        
        let thread = contract.get_the_thread(4);

        log!("thread numb 4 for check  = {:?}", thread);
        
        
        log!("add second answer");

        //contract.add_answers(4, "+15".to_string());
        /*contract.add_answers(4, "laht sasat".to_string());*/

        //let answ = contract.get_thread_answers(4);
        //log!("answers  = {:?}", answ);





    }
}
*/