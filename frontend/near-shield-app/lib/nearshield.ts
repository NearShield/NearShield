import { Account } from 'near-api-js';
import { CONTRACT_ID } from '@/config/near';

export interface Campaign {
  id: number;
  owner: string;
  token: string | null;
  total_pool: string;
  remaining_pool: string;
  metadata: {
    name: string;
    description: string;
    repo_link?: string;
    scope?: string;
  };
  // ... other fields
}

export async function viewMethod(account: Account, method: string, args = {}) {
  return await account.viewFunction(CONTRACT_ID, method, args);
}

export async function callMethod(account: Account, method: string, args = {}, deposit?: string, gas?: string) {
  return await account.functionCall({
    contractId: CONTRACT_ID,
    methodName: method,
    args,
    attachedDeposit: deposit ? parseNearAmount(deposit) : '0',
    gas: gas || '30000000000000',
  });
}

// Convenience functions
export async function getCampaigns(account: Account, from = 0, limit = 10) {
  return viewMethod(account, 'get_campaigns', { from_index: from, limit });
}

export async function getCampaign(account: Account, id: number) {
  return viewMethod(account, 'get_campaign', { campaign_id: id });
}

export async function createCampaignNear(account: Account, input: any, depositNear: string) {
  return callMethod(account, 'create_campaign_near', input, depositNear);
}

export async function submitBug(account: Account, campaignId: number, input: any) {
  return callMethod(account, 'submit_bug', { campaign_id: campaignId, input });
}

// ... other methods
