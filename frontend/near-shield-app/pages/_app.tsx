import '@/styles/globals.css'
import type { AppProps } from 'next/app'
import { Toaster } from 'sonner'
import { WalletProvider } from '@/context/WalletContext' // we'll create this

export default function App({ Component, pageProps }: AppProps) {
  return (
    <WalletProvider>
      <Component {...pageProps} />
      <Toaster richColors position="top-right" />
    </WalletProvider>
  )
}
