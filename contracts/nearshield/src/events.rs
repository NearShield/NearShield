use near_sdk::{env, log};
use crate::*;

pub fn emit_campaign_created(campaign: &Campaign) {
    log!(format!(
        "EVENT_JSON:{}",
        serde_json::json!({
            "standard": "nearshield",
            "version": "1.0.0",
            "event": "campaign_created",
            "data": {
                "campaign_id": campaign.id,
                "owner": campaign.owner,
                "total_pool": campaign.total_pool.to_string(),
                "token": campaign.token,
                "name": campaign.metadata.name,
            }
        })
    ));
}

pub fn emit_submission_created(submission: &Submission) {
    log!(format!(
        "EVENT_JSON:{}",
        serde_json::json!({
            "standard": "nearshield",
            "version": "1.0.0",
            "event": "submission_created",
            "data": {
                "submission_id": submission.id,
                "campaign_id": submission.campaign_id,
                "submitter": submission.submitter,
                "severity_claim": submission.severity_claim,
            }
        })
    ));
}

pub fn emit_payout(campaign_id: u64, submission_id: u64, receiver: AccountId, gross: Balance, fee: Balance) {
    log!(format!(
        "EVENT_JSON:{}",
        serde_json::json!({
            "standard": "nearshield",
            "version": "1.0.0",
            "event": "payout",
            "data": {
                "campaign_id": campaign_id,
                "submission_id": submission_id,
                "receiver": receiver,
                "gross_reward": gross.to_string(),
                "platform_fee": fee.to_string(),
            }
        })
    ));
}

pub fn emit_pause_toggle(paused: bool) {
    log!(format!(
        "EVENT_JSON:{}",
        serde_json::json!({
            "standard": "nearshield",
            "version": "1.0.0",
            "event": "pause_toggle",
            "data": { "paused": paused }
        })
    ));
}

pub fn emit_campaign_cancelled(campaign_id: u64, refund_amount: Balance) {
    log!(format!(
        "EVENT_JSON:{}",
        serde_json::json!({
            "standard": "nearshield",
            "version": "1.0.0",
            "event": "campaign_cancelled",
            "data": {
                "campaign_id": campaign_id,
                "refund_amount": refund_amount.to_string(),
            }
        })
    ));
}
