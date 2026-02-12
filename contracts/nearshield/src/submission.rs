use near_sdk::{env, require, log, Promise};
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Submission {
    pub id: u64,
    pub campaign_id: u64,
    pub submitter: AccountId,
    pub title: String,
    pub description_hash: String,
    pub poc_link: String,
    pub severity_claim: u8,
    pub status: SubmissionStatus,
    pub review_comments: Option<String>,
    pub reward_amount: Option<Balance>,
    pub reviewer: Option<AccountId>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub enum SubmissionStatus {
    Pending,
    UnderReview,
    Accepted,
    Rejected,
    Duplicate,
    Informative,
}

impl Contract {
    pub fn submit_bug(&mut self, campaign_id: u64, input: SubmitBugInput) -> u64 {
        require!(!self.paused, "Contract paused");
        let campaign = self.campaigns.get(&campaign_id).expect("Campaign not found");
        require!(!campaign.cancelled, "Campaign cancelled");
        if let Some(end) = campaign.end_time {
            require!(env::block_timestamp_ms() < end, "Campaign ended");
        }

        let submission_id = self.next_submission_id;
        self.next_submission_id += 1;

        let submission = Submission {
            id: submission_id,
            campaign_id,
            submitter: env::predecessor_account_id(),
            title: input.title,
            description_hash: input.description_hash,
            poc_link: input.poc_link,
            severity_claim: input.severity_claim,
            status: SubmissionStatus::Pending,
            review_comments: None,
            reward_amount: None,
            reviewer: None,
            created_at: env::block_timestamp_ms(),
            updated_at: env::block_timestamp_ms(),
        };

        self.submissions.insert(&submission_id, &submission);

        // Link submission to campaign
        let mut list = self
            .campaign_submissions
            .get(&campaign_id)
            .unwrap_or_else(|| Vector::new(StorageKey::CampaignSubmissions { campaign_id }));
        list.push(&submission_id);
        self.campaign_submissions.insert(&campaign_id, &list);

        emit_submission_created(&submission);
        submission_id
    }

    #[payable]
    pub fn review_submission(
        &mut self,
        submission_id: u64,
        status: SubmissionStatus,
        reward_amount: Option<Balance>,
        comments: Option<String>,
    ) {
        require!(!self.paused, "Contract paused");
        let submission = self.submissions.get(&submission_id).expect("Submission not found");
        let campaign = self.campaigns.get(&submission.campaign_id).expect("Campaign not found");
        require!(
            env::predecessor_account_id() == campaign.owner,
            "Only campaign owner"
        );
        require!(!campaign.cancelled, "Campaign cancelled");
        require!(
            submission.status == SubmissionStatus::Pending || submission.status == SubmissionStatus::UnderReview,
            "Invalid status transition"
        );

        let mut submission = submission;
        submission.status = status.clone();
        submission.review_comments = comments;
        submission.reviewer = Some(env::predecessor_account_id());
        submission.updated_at = env::block_timestamp_ms();

        if status == SubmissionStatus::Accepted {
            let reward = reward_amount.expect("Reward amount required for acceptance");
            // Validate reward <= campaign.remaining_pool and <= severity max
            let severity_config = campaign
                .severity_levels
                .iter()
                .find(|s| s.id == submission.severity_claim)
                .expect("Invalid severity claim");
            let max_allowed = (campaign.remaining_pool * severity_config.max_reward_pct as u128) / 100;
            require!(reward <= max_allowed, "Reward exceeds max for this severity");

            submission.reward_amount = Some(reward);
            self.submissions.insert(&submission_id, &submission);
            
            // Process payout (separate function to avoid stack issues)
            self.process_payout(&campaign, submission_id, submission.submitter, reward);
        } else {
            self.submissions.insert(&submission_id, &submission);
        }
    }

    fn process_payout(&mut self, campaign: &Campaign, submission_id: u64, receiver: AccountId, gross_reward: Balance) {
        let fee = (gross_reward * campaign.platform_fee_percent as u128) / 100;
        let net_reward = gross_reward - fee;

        // Update campaign remaining pool
        let mut campaign = campaign.clone();
        campaign.remaining_pool -= gross_reward;
        self.campaigns.insert(&campaign.id, &campaign);

        // Update leaderboards
        self.update_finder_stats(&receiver, net_reward, 1, submission.severity_claim);
        self.update_project_stats(&campaign.owner, gross_reward, 0, 1);

        // Transfer fee to treasury
        if let Some(token) = &campaign.token {
            // NEP-141 payout
            ext_ft::ext(token.clone())
                .with_attached_deposit(1)
                .with_static_gas(Gas(10 * TGAS))
                .ft_transfer(receiver.clone(), net_reward, None);
            
            if fee > 0 {
                ext_ft::ext(token.clone())
                    .with_attached_deposit(1)
                    .with_static_gas(Gas(5 * TGAS))
                    .ft_transfer(self.treasury.clone(), fee, None);
            }
        } else {
            // NEAR payout
            Promise::new(receiver).transfer(net_reward);
            if fee > 0 {
                Promise::new(self.treasury.clone()).transfer(fee);
            }
        }

        emit_payout(campaign.id, submission_id, receiver, gross_reward, fee);
    }
}
