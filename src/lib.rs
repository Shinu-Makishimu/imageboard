use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::{near_bindgen, AccountId, env};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Thread {
    pub author: AccountId,
    pub text: String,
    pub is_closed: bool,
    pub answers: UnorderedMap<i32, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
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

        if self.threads.len() == 500 {
            let mut vector_key = self.threads.keys_as_vector().to_vec();
            vector_key.sort();
            let key = vector_key.first().unwrap();
            self.remove_thread(&key);

        }


        let answers: UnorderedMap<i32, String> = UnorderedMap::new(b"m");

        let message = Thread{
            author, 
            text, 
            is_closed,
            answers,
        };

        self.threads.insert(&threads_count, &message);
    }
 
    pub fn get_threads(&self) -> Vec<(i32, Thread)> {
        self.threads.to_vec()     
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
        let index = self.moderators.iter().position(|x| &x.as_str() == &user_id.as_str()).unwrap();
        self.moderators.swap_remove(index as u64);

    }
}



impl Thread {
    pub fn add_answers(&mut self, text: String) -> String {
        if self.is_closed {
            return "thread is closed".to_string();
        }

        let mut count = self.answers.len() as i32;
        if count == 0 {
            self.answers.insert(&1,&text);
            return "succes".to_string();
        } else if count == 500 {
            return "thread is closed".to_string();
        } else {
            count +=1;
            self.answers.insert(&count, &text);
            return "succes".to_string();
        }

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
