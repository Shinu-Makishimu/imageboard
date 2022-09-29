use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{near_bindgen, AccountId, env};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Thread {
    pub author: AccountId,
    pub text: String,
    pub is_closed: bool,
//    pub answers: Vector<Answers>,
}

/*
pub struct Answers {
    pub posts: ???
}
*/


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ImageBoard {
    threads: Vector<Thread>,
    owner: AccountId,
    moderators: Vector<AccountId>,
}


impl Default for ImageBoard{
    fn default() -> Self {
        let owner = env::predecessor_account_id();
        Self { 
            threads: Vector::new(b"i"), 
            owner, 
            moderators: Vector::new(b"m"),
        
        }
    }
}


#[near_bindgen]
impl ImageBoard{
    pub fn add_thread(&mut self, text: String) {
        let is_closed: bool = false;
        let author = env::predecessor_account_id();

        let message = Thread{author, text, is_closed};

        self.threads.push(&message);
    }

    pub fn get_threads(&self) -> Vec<Thread> {
        self.threads.iter().take(50 as usize).collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_thread() {
        let mut contract = ImageBoard::default();
        contract.add_thread("there are all dead".to_string());
        let thread = &contract.get_threads()[0];
        
        assert_eq!(thread.text, "there are all dead".to_string())
    }
}
