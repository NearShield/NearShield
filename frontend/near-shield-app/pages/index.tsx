import { Button } from '@/components/ui/button'
import Link from 'next/link'
import { useWalletContext } from '@/context/WalletContext'

export default function Home() {
  const { account, signIn } = useWalletContext()
  
  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-50 to-slate-100 dark:from-slate-950 dark:to-slate-900">
      <div className="container mx-auto px-4 py-16">
        <nav className="flex justify-between items-center mb-16">
          <h1 className="text-2xl font-bold text-primary">NEARShield</h1>
          <div className="space-x-4">
            {account ? (
              <span className="text-sm text-muted-foreground">
                {account.accountId}
              </span>
            ) : (
              <Button onClick={signIn}>Connect Wallet</Button>
            )}
          </div>
        </nav>
        
        <div className="max-w-3xl mx-auto text-center">
          <h2 className="text-5xl font-bold tracking-tight mb-6">
            Decentralized Bug Bounties
            <span className="text-primary block mt-2">on NEAR Protocol</span>
          </h2>
          <p className="text-xl text-muted-foreground mb-8">
            Launch campaigns, submit findings, get paid automatically.
            Trustless, transparent, and NEAR‑native.
          </p>
          <div className="flex gap-4 justify-center">
            <Button asChild size="lg">
              <Link href="/campaigns">Explore Campaigns</Link>
            </Button>
            <Button asChild variant="outline" size="lg">
              <Link href="/campaigns/create">Start a Campaign</Link>
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}
