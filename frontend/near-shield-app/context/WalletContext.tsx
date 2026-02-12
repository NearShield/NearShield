import { createContext, useContext } from 'react'
import { useWallet } from '@/hooks/useWallet'

const WalletContext = createContext<ReturnType<typeof useWallet> | null>(null)

export const WalletProvider = ({ children }: { children: React.ReactNode }) => {
  const wallet = useWallet()
  return (
    <WalletContext.Provider value={wallet}>
      {children}
    </WalletContext.Provider>
  )
}

export const useWalletContext = () => {
  const ctx = useContext(WalletContext)
  if (!ctx) throw new Error('useWalletContext must be used inside WalletProvider')
  return ctx
}
