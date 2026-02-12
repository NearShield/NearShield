import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Card, CardContent } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { useWallet } from '@/hooks/useWallet';
import { createCampaignNear } from '@/lib/nearshield';
import { useRouter } from 'next/router';
import { toast } from 'sonner';

const formSchema = z.object({
  name: z.string().min(3, 'Name too short'),
  description: z.string().min(10),
  repo_link: z.string().url().optional(),
  scope: z.string().optional(),
  rules: z.string().optional(),
  contact: z.string().optional(),
  severity_levels: z.array(z.object({
    name: z.string(),
    max_reward_pct: z.coerce.number().min(1).max(100),
  })).min(1),
  campaign_type: z.enum(['Public', 'Private']),
  end_time: z.string().optional(),
  deposit_near: z.string().min(1, 'Deposit amount required'),
});

export default function CreateCampaign() {
  const { account, signIn } = useWallet();
  const router = useRouter();

  const form = useForm({
    resolver: zodResolver(formSchema),
    defaultValues: {
      severity_levels: [
        { name: 'Critical', max_reward_pct: 60 },
        { name: 'High', max_reward_pct: 30 },
        { name: 'Medium', max_reward_pct: 15 },
        { name: 'Low', max_reward_pct: 5 },
      ],
      campaign_type: 'Public',
    },
  });

  async function onSubmit(values: any) {
    if (!account) {
      signIn();
      return;
    }
    try {
      const deposit = values.deposit_near;
      delete values.deposit_near;
      await createCampaignNear(account, values, deposit);
      toast.success('Campaign created successfully!');
      router.push('/campaigns');
    } catch (err) {
      toast.error('Creation failed: ' + err.message);
    }
  }

  return (
    <div className="container max-w-4xl py-10">
      <h1 className="text-3xl font-bold mb-6">Create Bug Bounty Campaign</h1>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <Card>
            <CardContent className="pt-6 space-y-4">
              <FormField ... name="name" render={...} />
              <FormField ... name="description" render={...} />
              <div className="grid grid-cols-2 gap-4">
                <FormField ... name="repo_link" render={...} />
                <FormField ... name="contact" render={...} />
              </div>
              <FormField ... name="scope" render={...} />
              <FormField ... name="rules" render={...} />
              <FormField ... name="campaign_type" render={...} />
              <FormField ... name="end_time" render={...} />
              <div>
                <label className="text-sm font-medium">Severity Levels & Max % of Pool</label>
                {form.watch('severity_levels').map((_, idx) => (
                  <div key={idx} className="flex gap-4 mt-2">
                    <FormField name={`severity_levels.${idx}.name`} render={...} />
                    <FormField name={`severity_levels.${idx}.max_reward_pct`} render={...} />
                  </div>
                ))}
              </div>
              <FormField name="deposit_near" render={({ field }) => (
                <FormItem>
                  <FormLabel>Deposit (NEAR)</FormLabel>
                  <FormControl>
                    <Input type="number" step="0.001" placeholder="e.g. 100" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )} />
            </CardContent>
          </Card>
          <Button type="submit" className="w-full" size="lg">
            {account ? 'Create & Deposit' : 'Connect Wallet to Create'}
          </Button>
        </form>
      </Form>
    </div>
  );
}
