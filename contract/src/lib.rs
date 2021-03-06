/*
 * NEAR DNS
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, StorageUsage};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearDns {
    a_records: UnorderedMap<AccountId, String>,
    aaaa_records: UnorderedMap<AccountId, String>,
    contenthash_records: UnorderedMap<AccountId, String>,
    txt_records: UnorderedMap<AccountId, String>,
    cost_of_insertion: StorageUsage,
}

#[near_bindgen]
impl NearDns {
    fn measure_cost_of_insertion(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));
        let tmp_account_record = "a".repeat(64);
        // a_records, aaaa_records, contenthash_records, and txt_records should all cost
        // the same amount of storage, will use a_records
        self.a_records.insert(&tmp_account_id, &tmp_account_record);
        self.cost_of_insertion = env::storage_usage() - initial_storage_usage;
        self.a_records.remove(&tmp_account_id);
    }

    fn check_cost_of_insertion(&self) -> bool {
        env::attached_deposit() < self.cost_of_insertion as u128
    }

    #[init]
    pub fn new() -> Self {
        let mut this = Self {
            a_records: UnorderedMap::new(b"a".to_vec()),
            aaaa_records: UnorderedMap::new(b"b".to_vec()),
            contenthash_records: UnorderedMap::new(b"c".to_vec()),
            txt_records: UnorderedMap::new(b"t".to_vec()),
            cost_of_insertion: 0
        };
        this.measure_cost_of_insertion();
        this
    }

    pub fn get_a(&self, account_id: AccountId) -> String {
        match self.a_records.get(&account_id) {
            Some(record) => record,
            None => "".to_string(),
        }
    }

    pub fn get_aaaa(&self, account_id: AccountId) -> String {
        match self.aaaa_records.get(&account_id) {
            Some(record) => record,
            None => "".to_string(),
        }
    }

    pub fn get_content_hash(&self, account_id: AccountId) -> String {
        match self.contenthash_records.get(&account_id) {
            Some(record) => record,
            None => "".to_string(),
        }
    }

    pub fn get_txt(&self, account_id: AccountId) -> String {
        match self.txt_records.get(&account_id) {
            Some(record) => record,
            None => "".to_string(),
        }
    }

    #[payable]
    pub fn set_a(&mut self, a_record: String) {
        if self.check_cost_of_insertion() {
            env::panic_str(format!(
                "attached deposit '{}' < cost_of_insertion '{}'",
                env::attached_deposit(), self.cost_of_insertion,
            )
            .as_str(),)
        }

        let account_id = env::predecessor_account_id();

        // set A record for account_id
        let action = if self.a_records.get(&account_id).is_some() {
            "update"
        } else {
            "set"
        };
        self.a_records.insert(&account_id, &a_record);

        env::log_str(
            format!(
                "{} A record '{}' for account '{}'",
                action, a_record, account_id,
            )
            .as_str(),
        );
    }

    #[payable]
    pub fn set_aaaa(&mut self, aaaa_record: String) {
        if self.check_cost_of_insertion() {
            env::panic_str(format!(
                "attached deposit '{}' < cost_of_insertion '{}'",
                env::attached_deposit(), self.cost_of_insertion,
            )
            .as_str(),)
        }

        let account_id = env::predecessor_account_id();

        // set AAAA record for account_id
        let action = if self.aaaa_records.get(&account_id).is_some() {
            "update"
        } else {
            "set"
        };
        self.aaaa_records.insert(&account_id, &aaaa_record);

        env::log_str(
            format!(
                "{} AAAA record '{}' for account '{}'",
                action, aaaa_record, account_id,
            )
            .as_str(),
        );
    }

    #[payable]
    pub fn set_content_hash(&mut self, content_hash: String) {
        if self.check_cost_of_insertion() {
            env::panic_str(format!(
                "attached deposit '{}' < cost_of_insertion '{}'",
                env::attached_deposit(), self.cost_of_insertion,
            )
            .as_str(),)
        }

        let account_id = env::predecessor_account_id();

        // set Content Hash record for account_id
        let action = if self.contenthash_records.get(&account_id).is_some() {
            "update"
        } else {
            "set"
        };

        self.contenthash_records.insert(&account_id, &content_hash);

        env::log_str(
            format!(
                "{} content_hash record '{}' for account '{}'",
                action, content_hash, account_id,
            )
            .as_str(),
        );
    }

    #[payable]
    pub fn set_txt(&mut self, txt_record: String) {
        if self.check_cost_of_insertion() {
            env::panic_str(format!(
                "attached deposit '{}' < cost_of_insertion '{}'",
                env::attached_deposit(), self.cost_of_insertion,
            )
            .as_str(),)
        }
        
        let account_id = env::predecessor_account_id();

        // set TXT record for account_id
        let action = if self.txt_records.get(&account_id).is_some() {
            "update"
        } else {
            "set"
        };
        self.txt_records.insert(&account_id, &txt_record);

        env::log_str(
            format!(
                "{} TXT record '{}' for account '{}'",
                action, txt_record, account_id,
            )
            .as_str(),
        );
    }
}

/*
 * To run from contract directory:
 * cargo test -- --nocapture
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn carol() -> AccountId {
        AccountId::new_unchecked("carol_near".to_string())
    }

    fn get_context() -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(carol())
            .attached_deposit(10000)
            .build()
    }

    #[test]
    fn set_then_get_a() {
        let context = get_context();
        testing_env!(context);
        let mut contract = NearDns::new();
        contract.set_a("127.0.0.1".to_string());
        assert_eq!("127.0.0.1".to_string(), contract.get_a(carol()));
    }

    #[test]
    fn set_then_get_aaaa() {
        let context = get_context();
        testing_env!(context);
        let mut contract = NearDns::new();
        contract.set_aaaa("::1".to_string());
        assert_eq!("::1".to_string(), contract.get_aaaa(carol()));
    }

    #[test]
    fn set_then_get_content_hash() {
        let context = get_context();
        testing_env!(context);
        let mut contract = NearDns::new();
        contract.set_content_hash("ipfs_cid".to_string());
        assert_eq!("ipfs_cid".to_string(), contract.get_content_hash(carol()));
    }

    #[test]
    fn set_then_get_txt() {
        let context = get_context();
        testing_env!(context);
        let mut contract = NearDns::new();
        contract.set_txt("txt".to_string());
        assert_eq!("txt".to_string(), contract.get_txt(carol()));
    }
}
