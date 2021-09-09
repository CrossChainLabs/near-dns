/*
 * NEAR DNS
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct NearDns {
    a_records: UnorderedMap<String, String>,
    aaaa_records: UnorderedMap<String, String>,
    contenthash_records: UnorderedMap<String, String>,
    txt_records: UnorderedMap<String, String>
}

#[near_bindgen]
impl NearDns {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The NEAR_DNS is already initialized");
        Self {
            a_records: UnorderedMap::new(b"a".to_vec()),
            aaaa_records: UnorderedMap::new(b"a".to_vec()),
            contenthash_records: UnorderedMap::new(b"a".to_vec()),
            txt_records: UnorderedMap::new(b"a".to_vec())
        }
    }

    pub fn get_a(&self, account_id: String) -> String {
            match self.a_records.get(&account_id) {
                Some(record) => record,
                None => "".to_string(),
            }
    }

    pub fn get_aaaa(&self, account_id: String) -> String {
        match self.aaaa_records.get(&account_id) {
            Some(record) => record,
            None => "".to_string(),
        }

    }

    pub fn get_content_hash(&self, account_id: String) -> String {
        match self.contenthash_records.get(&account_id) {
            Some(record) => record,
            None => "".to_string(),
        }
    }

    pub fn get_txt(&self, account_id: String) -> String {
        match self.txt_records.get(&account_id) {
            Some(record) => record,
            None => "".to_string(),
        }
    }

    pub fn set_a(&mut self, a_record: String) {
        //let account_id = env::signer_account_id();
        let account_id = env::predecessor_account_id();
        let mut action = "set";

        // set A record for account_id
        let empty_record = "".to_string();
        let mut record = self.a_records.get(&a_record).unwrap_or(empty_record);
        if !record.is_empty() {
            action = "update";
        }
        record = a_record.clone();
        self.a_records.insert(&account_id, &record);
        
        env::log(format!("{} A record '{}' for account '{}'", action, a_record, account_id,).as_bytes());
    }

    pub fn set_aaaa(&mut self, aaaa_record: String) {
        //let account_id = env::signer_account_id();
        let account_id = env::predecessor_account_id();
        let mut action = "set";

        // set AAAA record for account_id
        let empty_record = "".to_string();
        let mut record = self.aaaa_records.get(&aaaa_record).unwrap_or(empty_record);
        if !record.is_empty() {
            action = "update";
        }
        record = aaaa_record.clone();
        self.aaaa_records.insert(&account_id, &record);

        env::log(format!("{} AAAA record '{}' for account '{}'", action, aaaa_record, account_id,).as_bytes());
    }

    pub fn set_content_hash(&mut self, content_hash: String) {
        //let account_id = env::signer_account_id();
        let account_id = env::predecessor_account_id();
        let mut action = "set";

        // set Content Hash record for account_id
        let empty_record = "".to_string();
        let mut record = self.contenthash_records.get(&content_hash).unwrap_or(empty_record);
        if !record.is_empty() {
            action = "update";
        }
        record = content_hash.clone();
        self.contenthash_records.insert(&account_id, &record);
       
        env::log(format!("{} content_hash record '{}' for account '{}'", action, content_hash, account_id,).as_bytes());
    }

    pub fn set_txt(&mut self, txt_record: String) {
        //let account_id = env::signer_account_id();
        let account_id = env::predecessor_account_id();
        let mut action = "set";

        // set TXT record for account_id
        let empty_record = "".to_string();
        let mut record = self.txt_records.get(&txt_record).unwrap_or(empty_record);
        if !record.is_empty() {
            action = "update";
        }
        record = txt_record.clone();
        self.txt_records.insert(&account_id, &record);

        env::log(format!("{} TXT record '{}' for account '{}'", action, txt_record, account_id,).as_bytes());
    }
}

/*
 * To run from contract directory:
 * cargo test -- --nocapture
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_a() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = NearDns::default();
        contract.set_a("127.0.0.1".to_string());
        assert_eq!(
            "127.0.0.1".to_string(),
            contract.get_a("carol_near".to_string())
        );
    }

    #[test]
    fn set_then_get_aaaa() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = NearDns::default();
        contract.set_aaaa("::1".to_string());
        assert_eq!(
            "::1".to_string(),
            contract.get_aaaa("carol_near".to_string())
        );
    }

    #[test]
    fn set_then_get_content_hash() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = NearDns::default();
        contract.set_content_hash("ipfs_cid".to_string());
        assert_eq!(
            "ipfs_cid".to_string(),
            contract.get_content_hash("carol_near".to_string())
        );
    }

    #[test]
    fn set_then_get_txt() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = NearDns::default();
        contract.set_txt("txt".to_string());
        assert_eq!(
            "txt".to_string(),
            contract.get_txt("carol_near".to_string())
        );
    }
}
