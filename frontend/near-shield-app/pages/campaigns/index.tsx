import { useEffect, useState } from 'react'
import { useWalletContext } from '@/context/WalletContext'
import { getCampaigns } from '@/lib/nearshield'
import { Campaign } from '@/types'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import Link from 'next/link'

export default function Campaigns() {
  const { account } = useWalletContext()
  const [campaigns, setCampaigns] = useState<Campaign[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    if (!account) return
    getCampaigns(account, 0, 20).then(setCampaigns).finally(() => setLoading(false))
  }, [account])

  return (
    <div className="container mx-auto py-10">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">Active Bug Bounties</h1>
        <Button asChild>
          <Link href="/campaigns/create">+ New Campaign</Link>
        </Button>
      </div>
      
      {!account && (
        <div className="text-center py-12">
          <p className="text-muted-foreground mb-4">Connect your wallet to view campaigns</p>
          <Button onClick={useWalletContext().signIn}>Connect</Button>
        </div>
      )}

      {loading && <p>Loading campaigns...</p>}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {campaigns.map((c) => (
          <Card key={c.id} className="hover:shadow-lg transition">
            <CardHeader>
              <CardTitle className="line-clamp-1">{c.metadata.name}</CardTitle>
              <p className="text-sm text-muted-foreground">by {c.owner}</p>
            </CardHeader>
            <CardContent>
              <p className="text-sm line-clamp-2 mb-4">{c.metadata.description}</p>
              <div className="flex justify-between text-sm">
                <span>💰 Pool: {c.remaining_pool} Ⓝ</span>
                <span>📦 {c.cancelled ? 'Cancelled' : 'Active'}</span>
              </div>
              <Button asChild className="w-full mt-4">
                <Link href={`/campaigns/${c.id}`}>View Details</Link>
              </Button>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  )
}
