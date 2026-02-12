import { Web3Storage } from 'web3.storage' // or use any IPFS service

const client = new Web3Storage({ token: process.env.NEXT_PUBLIC_WEB3STORAGE_TOKEN || '' })

export async function uploadToIPFS(content: string): Promise<string> {
  const blob = new Blob([content], { type: 'text/plain' })
  const files = [new File([blob], 'description.txt')]
  const cid = await client.put(files)
  return cid
}
