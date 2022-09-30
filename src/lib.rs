use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
//use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, AccountId, env};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Debug)]
pub struct Thread {
    pub author: AccountId,
    pub text: String,
    pub is_closed: bool,
//    pub answers: Vector<Answers>,
}
/*
pub struct Answers {
    pub posts: 
}
*/



#[near_bindgen]
//#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[derive(BorshDeserialize, BorshSerialize,)]
//#[serde(crate = "near_sdk::serde")]
pub struct ImageBoard {
    threads: UnorderedMap<i32, Thread>,
    owner: AccountId,
    moderators: Vector<AccountId>,
    threads_count: i32,

}


impl Default for ImageBoard{
    fn default() -> Self {
        let owner = env::predecessor_account_id();
        let threads_count: i32 = 0;
        Self { 
            threads: UnorderedMap::new(b"s"), 
            owner, 
            moderators: Vector::new(b"m"),
            threads_count,
        
        }
    }
}



#[near_bindgen]
impl ImageBoard{
    pub fn add_thread(&mut self, text: String) {
        let is_closed: bool = false;
        let author = env::predecessor_account_id();
        self.threads_count += 1;
        let threads_count = self.threads_count;

        let message = Thread{author, text, is_closed};

        self.threads.insert(&threads_count, &message);
    }

    pub fn get_threads(&self) -> Vec<(i32, Thread)> {
        self.threads.to_vec()     
    }

    pub fn add_moder(){

    }
}




#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_thread() {
        let mut contract = ImageBoard::default();
        contract.add_thread("there are all dead".to_string());
        contract.add_thread("there are all dead".to_string());
        contract.add_thread("there are all dead".to_string());

        let thread = &contract.get_threads()[2];
        let (key, sample) = thread;
        let one = 3;
        assert_eq!(key, &one);
        assert_eq!(sample.text, "there are all dead".to_string());
    }
}
