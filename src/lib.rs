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
    // pub fn delete_job(&self){
    //     let _account_id = env::signer_account_id();


    // }
    // pub fn update_job(){
    //     let _account_id = env::signer_account_id();
    //     let _a_job = JobsMarket::default();

    // }

    // pub fn delete_job(&self) -> Option<Jobs>{
    //     let account_id = env::signer_account_id();
    //     return self.jobs.remove(&account_id);
    // }

    // pub fn get_job(&self) -> Vec<Jobs> {
    //     let account_id = env::signer_account_id();
    //     let mut jobs = Vec::new();
    //     for job in self.jobs.iter(&account_id) {
    //         jobs.push(job.clone());
    //     }
    //     return jobs;
    // }
    // pub fn get_jobs(&self) -> Option<Jobs> {
    //     let account_id = env::signer_account_id();
    //     return self.jobs.get(&account_id);
    // }


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
        //self.jobs.insert(&account_id, &a_job);
        // self.jobs.insert(&account_id, &a_job);
        if account_id != account_id{
            env::log_str("updated");
            self.jobs.insert(&account_id, &a_job);
            
        } else {
            env::log_str("You are not allowed to update this job");
        } 
        //update
        //if env account id !=  account id ya job reject 
    }
    pub fn delete_job(&mut self){
        // get account id
        let account_id = env::signer_account_id();
        // get job from lookup map
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
    use std::task::Context;

    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }


    // TESTS HERE
    // #[test]
    // fn get_nonexistence_job(){
    //     let context = get_context(true);
    //     testing_env!(context);
    //     let job = JobsMarket::default();
    //     assert_eq!(None, job.get_job("software".to_string()));
    // }
    #[test]
    fn get_job(){
        let _account_id = env::signer_account_id();
        //let context = get_context(predecessor);
        //testing_env!(context);
        let a_job = Jobs{
            title: "software".to_string(),
            description : "software".to_string(),
            date : "11-2-2020".to_string(),
            company : "software".to_string(),
            location : "software".to_string() ,
            email : "software".to_string()
        };
        // let job = JobsMarket::default();
        assert_eq!(a_job.title, "software".to_string());
        assert_eq!(a_job.description, "software".to_string());
        assert_eq!(a_job.date, "11-2-2020".to_string());
        assert_eq!(a_job.company, "software".to_string());
        assert_eq!(a_job.location, "software".to_string());
        assert_eq!(a_job.email, "software".to_string());
        

    }
    #[test]
    fn set_job(){
        let _account_id = env::signer_account_id();
        let mut job = JobsMarket::default();
        job.set_job("software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());
        let _a_job = job.get_job();
        //assert_eq!(a_job.title, "software".to_string());
        // assert_eq!(a_job.description, "software".to_string());
        // assert_eq!(a_job.date, "11-2-2020".to_string());
        // assert_eq!(a_job.company, "software".to_string());
        // assert_eq!(a_job.location, "software".to_string());
        // assert_eq!(a_job.email, "software".to_string());

        //let a_job = JobsMarket::set_job(&self, "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string(), "software".to_string());

    }
    #[test]
    fn delete_job(){
        let _account_id = env::signer_account_id();
        let job = JobsMarket::default();
        let _a_job = job.get_job();
        //return job.remove(a_job);
    }
    #[test]
    fn update_job(){
        let _account_id = env::signer_account_id();
        let job = JobsMarket::default();
        let _a_job = job.get_job();
    }
    // #[test]
    // fn set_job() {
    //     let context = get_context(AccountId);
    //     testing_env!(context);
    //     let jobs = JobsMarket::default();
    //     jobs.set_job("Software Engineer".to_string(),
    //         "We are looking for a software engineer to join our team".to_string(),
    //         "2020-01-01".to_string(),
    //         "Google".to_string(),
    //         "New York".to_string(),
    //         "dalmas@gmail.com".to_string());
    //     let a_job = jobs.get_job();
    //     // assert_eq!(a_job.len(), 1);
    //     // assert_eq!(a_job.title, "Software Engineer".to_string());
    //     // assert_eq!(jobs.description, "We are looking for a software engineer to join our team".to_string());
    //     // assert_eq!(jobs.date, "2020-01-01".to_string());
    //     // assert_eq!(jobs.company, "Google".to_string());
    //     // assert_eq!(jobs.location, "New York".to_string());
    //     // assert_eq!(jobs.email, "dalmas@gmail.com".to_string());
    // }

    
}
