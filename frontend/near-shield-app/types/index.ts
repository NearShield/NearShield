export interface Campaign {
  id: number
  owner: string
  token: string | null
  total_pool: string
  remaining_pool: string
  severity_levels: SeverityLevel[]
  platform_fee_percent: number
  campaign_type: 'Public' | 'Private'
  metadata: {
    name: string
    description: string
    repo_link?: string
    scope?: string
    rules?: string
    contact?: string
  }
  start_time: number
  end_time?: number
  cancelled: boolean
}

export interface SeverityLevel {
  id: number
  name: string
  max_reward_pct: number
}

export interface Submission {
  id: number
  campaign_id: number
  submitter: string
  title: string
  description: string;
  poc_link: string
  severity_claim: number
  status: 'Pending' | 'UnderReview' | 'Accepted' | 'Rejected' | 'Duplicate' | 'Informative'
  reward_amount?: string
  reviewer?: string
  created_at: number
  updated_at: number
}
