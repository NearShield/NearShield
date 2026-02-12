use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue, BorshStorageKey,
    CryptoHash, Gas, PromiseResult, require, log,
};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

mod campaign;
mod submission;
mod leaderboard;
mod owner;
mod deposit;
mod events;
mod types;

use campaign::*;
use submission::*;
use leaderboard::*;
use owner::*;
use deposit::*;
use events::*;
use types::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Campaigns
    campaigns: UnorderedMap<u64, Campaign>,
    next_campaign_id: u64,
    // Submissions
    submissions: UnorderedMap<u64, Submission>,
    next_submission_id: u64,
    // Per‑campaign submission IDs
    campaign_submissions: LookupMap<u64, Vector<u64>>,
    // Leaderboards
    finder_stats: UnorderedMap<AccountId, FinderStats>,
    project_stats: UnorderedMap<AccountId, ProjectStats>,
    // Platform admin
    admin: AccountId,
    treasury: AccountId,
    paused: bool,
}

// Storage keys for collections
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Campaigns,
    Submissions,
    CampaignSubmissions { campaign_id: u64 },
    FinderStats,
    ProjectStats,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(admin: AccountId, treasury: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            campaigns: UnorderedMap::new(StorageKey::Campaigns),
            next_campaign_id: 1,
            submissions: UnorderedMap::new(StorageKey::Submissions),
            next_submission_id: 1,
            campaign_submissions: LookupMap::new(b"c"),
            finder_stats: UnorderedMap::new(StorageKey::FinderStats),
            project_stats: UnorderedMap::new(StorageKey::ProjectStats),
            admin,
            treasury,
            paused: false,
        }
    }

    // --------------
    // View methods
    // --------------

    pub fn get_campaign(&self, campaign_id: u64) -> Option<Campaign> {
        self.campaigns.get(&campaign_id)
    }

    pub fn get_campaigns(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Campaign> {
        let keys = self.campaigns.keys_as_vector();
        let from = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(10);
        (from..std::cmp::min(from + limit, keys.len()))
            .filter_map(|i| keys.get(i).and_then(|id| self.campaigns.get(&id)))
            .collect()
    }

    pub fn get_submission(&self, submission_id: u64) -> Option<Submission> {
        self.submissions.get(&submission_id)
    }

    pub fn get_campaign_submissions(&self, campaign_id: u64, from: Option<u64>, limit: Option<u64>) -> Vec<Submission> {
        if let Some(list) = self.campaign_submissions.get(&campaign_id) {
            let from = from.unwrap_or(0);
            let limit = limit.unwrap_or(10);
            (from..std::cmp::min(from + limit, list.len()))
                .filter_map(|i| list.get(i).and_then(|id| self.submissions.get(&id)))
                .collect()
        } else {
            vec![]
        }
    }

    // Leaderboard views
    pub fn get_top_finders(&self, limit: u32) -> Vec<(AccountId, FinderStats)> {
        self.finder_stats
            .iter()
            .take(limit as usize)
            .map(|(account, stats)| (account, stats))
            .collect()
    }

    pub fn get_top_projects(&self, limit: u32) -> Vec<(AccountId, ProjectStats)> {
        self.project_stats
            .iter()
            .take(limit as usize)
            .map(|(account, stats)| (account, stats))
            .collect()
    }

    // --------------
    // Owner/admin methods (protected)
    // --------------
    pub fn set_paused(&mut self, paused: bool) {
        self.assert_admin();
        self.paused = paused;
        emit_pause_toggle(paused);
    }

    pub fn withdraw_fees(&mut self, amount: Option<Balance>, token: Option<AccountId>) -> Promise {
        self.assert_admin();
        require!(!self.paused, "Contract paused");

        let treasury = self.treasury.clone();
        match token {
            Some(token_account) => {
                // NEP-141 transfer
                ext_ft::ext(token_account)
                    .with_attached_deposit(1)
                    .with_static_gas(Gas(5 * TGAS))
                    .ft_transfer(treasury, amount.unwrap_or(env::account_balance()), None)
            }
            None => {
                // NEAR native
                let amount = amount.unwrap_or(env::account_balance());
                Promise::new(treasury).transfer(amount)
            }
        }
    }

    // --------------
    // NEP-141 receiver (for deposits)
    // --------------
    #[private]
    pub fn process_ft_deposit(&mut self, sender_id: AccountId, amount: u128, msg: String) {
        // Parses msg as JSON to create campaign
        let campaign_input: CreateCampaignInput = serde_json::from_str(&msg).expect("Invalid msg format");
        self.internal_create_campaign(
            sender_id,
            amount,
            Some(env::predecessor_account_id()), // token contract
            campaign_input,
        );
    }
}

// --------------------------------
// Fungible Token Receiver Impl
// --------------------------------
#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: u128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        require!(!self.paused, "Contract paused");
        // Use private method to process deposit
        self.process_ft_deposit(sender_id, amount, msg);
        PromiseOrValue::Value(U128(0)) // 0 refund = keep full amount
    }
}

// Helper to assert admin
impl Contract {
    fn assert_admin(&self) {
        require!(env::predecessor_account_id() == self.admin, "Only admin");
    }
}
