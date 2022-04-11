use borsh::{ BorshDeserialize, BorshSerialize };
use near_sdk::{
    env, near_bindgen, AccountId, Balance, Promise,
    collections::{ UnorderedMap },
    json_types::{ U128 },
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ONE_NEAR:u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Trust {
    pub owner_id: AccountId,
    pub balances: UnorderedMap<AccountId, Balance>,
}

impl Default for Trust {
    fn default() -> Self {
        panic!("should be initialized before usage")
    }
}

#[near_bindgen]
impl Trust {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid.");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            balances: UnorderedMap::new(b"balances".to_vec()),
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let deposit = env::attached_deposit();
        let account_id = env::signer_account_id();
        let mut balance = self.balances.get(&account_id).unwrap_or(0);
        balance += deposit;
        self.balances.insert(&account_id, &balance);
    }

    pub fn play(&mut self, opt_bal: u128) -> bool {
        let account_id = env::signer_account_id();
        let mut credits = self.balances.get(&account_id).unwrap_or(0);
        assert!(credits > 0, "no credits to play");

        let rng:u8 = *env::random_seed().get(0).unwrap() % 100;

        let acc_balance = 0;
        self.balances.insert(&account_id, &acc_balance);
        if rng < 45 {
            credits += opt_bal * ONE_NEAR * 965 / 100000;
            Promise::new(account_id).transfer(credits);
            return true;
        }
        return false;
    }

    pub fn get_balance(&self, account_id: AccountId) -> U128 {
        self.balances.get(&account_id).unwrap_or(0).into()
    }
}