use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::{near_bindgen, AccountId, env};


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
        self.threads_count += 1;
        let threads_count = self.threads_count;

        match self.threads.len() {
            500 => {
                let key = threads_count - 500;
                self.remove_thread(&key);
            },
            _=> {let answers: UnorderedMap<i32, String> = UnorderedMap::new(b"answers".to_vec());

                let message = Thread{
                    author, 
                    text, 
                    is_closed,
                    answers,
                };
    
                self.threads.insert(&threads_count, &message);
            },
        }
        
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
        if thread.is_closed {
            "thread is closed".to_string();
        }
        let mut count = thread.answers.len() as i32;

        match count {
            0 => {
                count = 1;
                thread.answers.insert(&count, &text); 
                "succes".to_string() 
            },
            500 => {
                thread.is_closed = true;
                "thread is closed".to_string()
            },
            _ => {
                count += 1;
                thread.answers.insert(&count, &text); 
                "succes".to_string()
            },
        }
    }

    pub fn get_thread_answers(&self, thread_number: i32) -> Vec<String> {
        let thread =  self.threads.get(&thread_number).unwrap();
        thread.answers.values_as_vector().to_vec()
    }
}


#[cfg(test)]
mod tests {
    use near_sdk::log;

    use super::*;
    
    #[test]
    fn add_thread() {
        let mut contract = ImageBoard::default();
        for _ in 1..6 {
            contract.add_thread("there are all dead".to_string());
            log!("shit {:?}", contract.get_threads().len());            
        }

        /*contract.add_thread("there are all dead".to_string());
        contract.add_thread("there are all dead".to_string());*/
        let threads = &contract.get_threads();

        /*let thread = &contract.get_threads()[2];
        let (key, sample) = thread;
        let one: i32 = 3;
        log!("count threads = {:?}", threads.len());
        assert_eq!(key, &one);
        assert_eq!(sample.text, "there are all dead".to_string());*/
        log!("count threads = {:?}", threads.len());
        assert_eq!(5, threads.len());

        let thread = contract.get_the_thread(4);
        log!("thread  = {:?}", thread);
        assert_eq!(thread, "there are all dead".to_string());



    }
}
