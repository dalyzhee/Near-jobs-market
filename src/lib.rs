use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{LookupMap};


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Jobs{
    title: String,
    description : String,
    date : String,
    company : String,
    location : String ,
    email : String 
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct JobsMarket {
    jobs : LookupMap<AccountId, Jobs>,
}

impl Default for JobsMarket {
    fn default() -> Self {
        Self {
            jobs: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl JobsMarket {
   
    pub fn set_job(&mut self, title: String,
        description : String,
        date : String,
        company : String,
        location : String ,
        email : String ) {
        let account_id = env::signer_account_id();
        let a_job = Jobs{
            title: title,
            description : description,
            date : date,
            company : company,
            location : location ,
            email : email
        };
        self.jobs.insert(&account_id, &a_job);
    }
    pub fn get_job(&self) -> Option<Jobs>{
        let account_id = env::signer_account_id();
        return self.jobs.get(&account_id);
    }
    pub fn update_job(&mut self){
        // get account id
        let account_id = env::signer_account_id();
        // get job from lokup map
        let _job = self.jobs.get(&account_id).unwrap();
        // update job
        let a_job = Jobs{
            title: "software".to_string(),
            description : "software is the best".to_string(),
            date : "12-3-2020".to_string(),
            company : "Andela".to_string(),
            location : "nairobi".to_string() ,
            email : "andela@gmail.com".to_string()
        };
        if account_id == account_id{
            env::log_str("updated");
            self.jobs.insert(&account_id, &a_job);
            
        } else {
            env::log_str("You are not allowed to update this job");
        } 
        //update
        //if env account id !=  account id ya job reject 
    }
    pub fn delete_job(&mut self){
        let account_id = env::signer_account_id();
        let _job = self.jobs.get(&account_id).unwrap();
        // delete job
        if account_id != account_id{
            env::log_str("You are not allowed to delete this job");
        } else {
            env::log_str("You have delete this job");
            self.jobs.remove(&account_id);
        }

    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(account: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(account);
        return builder;
    }


    // TESTS HERE
    #[test]
    fn get_nonexistence_job(){
        let user = AccountId::new_unchecked("dalyzhee.testnet".to_string());
        let _context = get_context(user.clone());
        let job = JobsMarket::default();
        let a_job = job.get_job();
        assert_eq!(true, a_job.is_none());
    }
    #[test]
    fn get_job(){
        let user = AccountId::new_unchecked("dalyzhee.testnet".to_string());
        let _context = get_context(user.clone());
        let mut job = JobsMarket::default();
        job.set_job("software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        let a_job = job.get_job();
        assert_eq!(true, a_job.is_some());       
    }
    #[test]
    fn set_job(){
        let user = AccountId::new_unchecked("dalyzhee.testnet".to_string());
        let _context = get_context(user.clone());
        let mut job = JobsMarket::default();
        job.set_job("software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        let a_job = job.get_job();
        assert_eq!(true, a_job.is_some());
    }
    #[test]
    fn delete_job(){
        let user = AccountId::new_unchecked("dalyzhee.testnet".to_string());
        let _context = get_context(user.clone());
        let mut job = JobsMarket::default();
        job.set_job("software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        job.delete_job();
        let a_job = job.get_job();
        assert_eq!(true, a_job.is_none());
    }
    // #[test]
    // fn update_job(){
    //     let user = AccountId::new_unchecked("dalyzhee.testnet".to_string());
    //     let _context = get_context(user.clone());
    //     let job = JobsMarket::default();
    //     let _a_job = job.get_job();
    // }
        
}
