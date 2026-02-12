use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance};
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct FinderStats {
    pub total_rewards_earned: Balance,
    pub total_bugs_found: u32,
    pub total_severity_score: u32, // sum of severity levels (0-...)
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct ProjectStats {
    pub total_rewards_paid: Balance,
    pub total_campaigns_created: u32,
    pub total_bugs_fixed: u32,
}

impl Contract {
    pub(crate) fn update_finder_stats(
        &mut self,
        finder: &AccountId,
        reward: Balance,
        bugs: u32,
        severity: u8,
    ) {
        let mut stats = self.finder_stats.get(finder).unwrap_or_default();
        stats.total_rewards_earned += reward;
        stats.total_bugs_found += bugs;
        stats.total_severity_score += severity as u32;
        self.finder_stats.insert(finder, &stats);
    }

    pub(crate) fn update_project_stats(
        &mut self,
        project: &AccountId,
        paid: Balance,
        campaigns: u32,
        bugs_fixed: u32,
    ) {
        let mut stats = self.project_stats.get(project).unwrap_or_default();
        stats.total_rewards_paid += paid;
        stats.total_campaigns_created += campaigns;
        stats.total_bugs_fixed += bugs_fixed;
        self.project_stats.insert(project, &stats);
    }

    // Call this when a campaign is created
    pub(crate) fn increment_project_campaigns(&mut self, project: &AccountId) {
        let mut stats = self.project_stats.get(project).unwrap_or_default();
        stats.total_campaigns_created += 1;
        self.project_stats.insert(project, &stats);
    }
}
