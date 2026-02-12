import { useEffect, useState } from 'react';
import { setupWalletSelector } from '@near-wallet-selector/core';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';
import { setupSender } from '@near-wallet-selector/sender';
import { setupHereWallet } from '@near-wallet-selector/here-wallet';
import type { WalletSelector, AccountState } from '@near-wallet-selector/core';
import { NETWORK_ID, CONTRACT_ID } from '@/config/near';

export function useWallet() {
  const [selector, setSelector] = useState<WalletSelector | null>(null);
  const [account, setAccount] = useState<AccountState | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setupWalletSelector({
      network: NETWORK_ID,
      modules: [
        setupMyNearWallet(),
        setupSender(),
        setupHereWallet(),
      ],
    }).then((selector) => {
      setSelector(selector);
      return selector.store.observable.subscribe((state) => {
        const accounts = state.accounts;
        if (accounts.length > 0) {
          setAccount(accounts[0]);
        } else {
          setAccount(null);
        }
        setLoading(false);
      });
    });
  }, []);

  const signIn = () => selector?.show();
  const signOut = () => selector?.signOut();

  return { selector, account, loading, signIn, signOut };
}
