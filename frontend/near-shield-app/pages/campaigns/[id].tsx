import { useRouter } from 'next/router'
import { useEffect, useState } from 'react'
import { useWalletContext } from '@/context/WalletContext'
import { getCampaign, submitBug } from '@/lib/nearshield'
import { Campaign } from '@/types'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Label } from '@/components/ui/label'
import { toast } from 'sonner'

export default function CampaignDetail() {
  const { account, signIn } = useWalletContext()
  const router = useRouter()
  const { id } = router.query
  const [campaign, setCampaign] = useState<Campaign | null>(null)
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)

  // Form state
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [pocLink, setPocLink] = useState('')
  const [severity, setSeverity] = useState(0)

  useEffect(() => {
    if (!id || !account) return
    getCampaign(account, Number(id)).then(setCampaign).finally(() => setLoading(false))
  }, [id, account])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!account) return signIn()
    if (!campaign) return

    setSubmitting(true)
    try {
      // Upload description to IPFS (optional – you can also store directly as text)
      const descriptionText = description
      await submitBug(account, campaign.id, {
  title,
  description: descriptionText,
        poc_link: pocLink,
  severity_claim: severity,
      })
      toast.success('Bug report submitted!')
      setTitle(''); setDescription(''); setPocLink(''); setSeverity(0)
    } catch (err: any) {
      toast.error(err.message)
    } finally {
      setSubmitting(false)
    }
  }

  if (!account) return <div className="container py-20 text-center">Please connect wallet</div>
  if (loading) return <div className="container py-20">Loading...</div>
  if (!campaign) return <div className="container py-20">Campaign not found</div>

  return (
    <div className="container max-w-4xl py-10">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">{campaign.metadata.name}</h1>
        <p className="text-muted-foreground mt-2">{campaign.metadata.description}</p>
      </div>

      <div className="grid md:grid-cols-3 gap-8">
        {/* Campaign Info */}
        <div className="md:col-span-1 space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Pool</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-2xl font-mono">{campaign.remaining_pool} Ⓝ</p>
              <p className="text-sm text-muted-foreground mt-1">Total: {campaign.total_pool} Ⓝ</p>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle>Severity Tiers</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2">
              {campaign.severity_levels.map((level) => (
                <div key={level.id} className="flex justify-between">
                  <span>{level.name}</span>
                  <span className="font-mono">Max {level.max_reward_pct}%</span>
                </div>
              ))}
            </CardContent>
          </Card>
        </div>

        {/* Submission Form */}
        <div className="md:col-span-2">
          <Card>
            <CardHeader>
              <CardTitle>Submit a Vulnerability</CardTitle>
            </CardHeader>
            <CardContent>
              <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                  <Label htmlFor="title">Title</Label>
                  <Input id="title" value={title} onChange={(e) => setTitle(e.target.value)} required />
                </div>
                <div>
                  <Label htmlFor="description">Description (will be uploaded to IPFS)</Label>
                  <Textarea id="description" rows={5} value={description} onChange={(e) => setDescription(e.target.value)} required />
                </div>
                <div>
                  <Label htmlFor="poc">Proof of Concept (URL)</Label>
                  <Input id="poc" value={pocLink} onChange={(e) => setPocLink(e.target.value)} placeholder="https://gist.github.com/..." />
                </div>
                <div>
                  <Label htmlFor="severity">Claimed Severity</Label>
                  <select
                    id="severity"
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    value={severity}
                    onChange={(e) => setSeverity(Number(e.target.value))}
                  >
                    {campaign.severity_levels.map((level) => (
                      <option key={level.id} value={level.id}>{level.name}</option>
                    ))}
                  </select>
                </div>
                <Button type="submit" disabled={submitting} className="w-full">
                  {submitting ? 'Submitting...' : 'Submit Bug'}
                </Button>
              </form>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
