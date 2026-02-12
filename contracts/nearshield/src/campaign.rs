use near_sdk::{env, AccountId, Balance, require, log};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Campaign {
    pub id: u64,
    pub owner: AccountId,
    pub token: Option<AccountId>, // None = NEAR, Some = NEP-141
    pub total_pool: Balance,
    pub remaining_pool: Balance,
    pub severity_levels: Vec<SeverityConfig>,
    pub platform_fee_percent: u8, // e.g. 1%
    pub campaign_type: CampaignType,
    pub metadata: CampaignMetadata,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub cancelled: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct SeverityConfig {
    pub id: u8,
    pub name: String,
    pub max_reward_pct: u8, // % of remaining pool (capped at config)
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct CampaignMetadata {
    pub name: String,
    pub description: String,
    pub repo_link: Option<String>,
    pub scope: Option<String>,
    pub rules: Option<String>,
    pub contact: Option<String>,
}

impl Contract {
    pub fn create_campaign_near(
        &mut self,
        input: CreateCampaignInput,
    ) -> u64 {
        require!(!self.paused, "Contract paused");
        let deposit = env::attached_deposit();
        require!(deposit > 0, "Attach at least 1 yoctoNEAR");

        self.internal_create_campaign(
            env::predecessor_account_id(),
            deposit,
            None, // NEAR
            input,
        )
    }

    // Internal: used by both NEAR deposit and FT deposit
    pub(crate) fn internal_create_campaign(
        &mut self,
        owner: AccountId,
        amount: Balance,
        token: Option<AccountId>,
        input: CreateCampaignInput,
    ) -> u64 {
        // Validate severity configs
        require!(!input.severity_levels.is_empty(), "At least one severity level");
        for level in &input.severity_levels {
            require!(level.max_reward_pct <= 100, "Max reward pct must be ≤100");
        }

        let campaign_id = self.next_campaign_id;
        self.next_campaign_id += 1;

        let severity_configs: Vec<SeverityConfig> = input
            .severity_levels
            .into_iter()
            .enumerate()
            .map(|(i, s)| SeverityConfig {
                id: i as u8,
                name: s.name,
                max_reward_pct: s.max_reward_pct,
            })
            .collect();

        let campaign = Campaign {
            id: campaign_id,
            owner,
            token,
            total_pool: amount,
            remaining_pool: amount,
            severity_levels: severity_configs,
            platform_fee_percent: 1, // hardcoded 1%
            campaign_type: input.campaign_type,
            metadata: CampaignMetadata {
                name: input.name,
                description: input.description,
                repo_link: input.repo_link,
                scope: input.scope,
                rules: input.rules,
                contact: input.contact,
            },
            start_time: env::block_timestamp_ms(),
            end_time: input.end_time,
            cancelled: false,
        };

        self.campaigns.insert(&campaign_id, &campaign);
        emit_campaign_created(&campaign);
        campaign_id
    }

    pub fn cancel_campaign(&mut self, campaign_id: u64) -> Promise {
        self.assert_campaign_owner(campaign_id);
        require!(!self.paused, "Contract paused");

        let mut campaign = self.campaigns.get(&campaign_id).expect("Campaign not found");
        require!(!campaign.cancelled, "Already cancelled");

        campaign.cancelled = true;
        let refund_amount = campaign.remaining_pool;
        campaign.remaining_pool = 0;
        self.campaigns.insert(&campaign_id, &campaign);

        emit_campaign_cancelled(campaign_id, refund_amount);

        // Refund owner
        if let Some(token) = &campaign.token {
            ext_ft::ext(token.clone())
                .with_attached_deposit(1)
                .with_static_gas(Gas(5 * TGAS))
                .ft_transfer(campaign.owner, refund_amount, None)
        } else {
            Promise::new(campaign.owner).transfer(refund_amount)
        }
    }

    fn assert_campaign_owner(&self, campaign_id: u64) {
        let campaign = self.campaigns.get(&campaign_id).expect("Campaign not found");
        require!(
            env::predecessor_account_id() == campaign.owner,
            "Only campaign owner"
        );
    }
}
