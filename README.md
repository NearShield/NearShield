NEARShield 🛡️

Decentralized Bug Bounty Platform for the NEAR Ecosystem

NEARShield empowers any NEAR project (dApp, protocol, smart contract) to launch and manage bug bounty campaigns entirely on‑chain. Security researchers submit findings, project owners review and pay rewards automatically – all through a trustless smart contract vault.

---

✨ Features

· Campaign Creation
  Project owners deposit NEAR or NEP‑141 tokens, define severity levels & max rewards, and set campaign scope. Funds are locked until paid out or refunded.
· Bug Submission & Review
  Researchers log in with NEAR wallet, submit reports with IPFS‑hosted PoC. Owners accept/reject, optionally set reward amount – contract handles the payout.
· Automated Payouts
  On approval, reward is transferred directly to the researcher’s wallet. A 1% platform fee is sent to the treasury contract.
· Leaderboards
  On‑chain rankings for top finders (by earnings) and top projects (by rewards paid).
· NEAR‑Native
  Built with Rust smart contract, Wallet Selector, NEP‑141 token support, and NEAR Intents‑ready architecture.
· Modern Frontend
  Next.js + TypeScript + Tailwind CSS + shadcn/ui – responsive, dark mode, wallet integration.
  📖 Usage Guide

For Project Owners

1. Connect NEAR wallet.
2. Navigate to “Create Campaign”.
3. Fill details: name, description, repo link, severity levels & max reward %.
4. Attach NEAR deposit (or send NEP‑141 tokens via transfer & call).
5. After creation, view campaign dashboard → review submissions → accept & pay.

For Security Researchers

1. Connect wallet.
2. Browse active campaigns.
3. Submit bug report with title, description (IPFS hash), PoC link, severity claim.
4. Wait for review – if accepted, reward is sent directly to your wallet.
