use near_sdk::{AccountId, Balance};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateCampaignInput {
    pub name: String,
    pub description: String,
    pub repo_link: Option<String>,
    pub scope: Option<String>,
    pub rules: Option<String>,
    pub contact: Option<String>,
    pub severity_levels: Vec<SeverityConfigInput>,
    pub campaign_type: CampaignType,
    pub end_time: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SeverityConfigInput {
    pub name: String,
    pub max_reward_pct: u8, // 0-100
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum CampaignType {
    Public,
    Private,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SubmitBugInput {
    pub title: String,
    pub description_hash: String, // IPFS CID
    pub poc_link: String,
    pub severity_claim: u8,
}
