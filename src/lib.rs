use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{LookupMap, Vector};

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
        let aJob = Jobs{
            title: title,
            description : description,
            date : date,
            company : company,
            location : location ,
            email : email
        };
        self.jobs.insert(&account_id, &aJob);
    }

    // pub fn set_job(&self)->Vector<Jobs>{

    // }
    pub fn update_job(&self){
        // get account id
        // get job from lokup map
        // update
        // if env account id !=  account id ya job reject
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
