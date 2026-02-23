NeuroWealth 💰

AI-Powered DeFi Yield Platform on Stellar

NeuroWealth is an autonomous AI investment agent that automatically manages and grows your crypto assets on the Stellar blockchain. Deposit once, let the AI find the best yield opportunities across Stellar's DeFi ecosystem — and withdraw anytime with no lock-ups.



Overview
Traditional savings accounts offer near-zero interest. Traditional DeFi is too complex for most users. NeuroWealth bridges the gap  a simple chat interface web  powered by an AI agent that autonomously deploys your funds into the highest-yielding, safest opportunities on Stellar.

Why Stellar?

Transaction fees of fractions of a penny — perfect for frequent AI-driven rebalancing
3–5 second finality — the AI can act on market changes instantly
Native DEX + Soroban smart contracts — composable, programmable yield strategies
Native USDC + XLM — borderless capital movement with no friction
Growing DeFi ecosystem — Blend (lending), Templar (borrowing), RWA protocols


Features
FeatureDescription🤖 AI AgentAutonomous 24/7 yield optimization across Stellar DeFi💬 Natural LanguageChat to deposit, withdraw, and check balances📈 Auto-RebalancingAgent shifts funds to best opportunities automatically🔐 Non-CustodialYour funds live in audited Soroban smart contracts⚡ Instant WithdrawalsNo lock-ups, no penalties, withdraw anytime📱 WhatsApp ReadyFull functionality through WhatsApp chat interface🌍 Global AccessNo geographic restrictions, no bank account required🛡️ Security FirstSoroban contracts with ReentrancyGuard and access controls

How It Works
1. User deposits USDC via web app
       ↓
2. Soroban vault contract receives and records the deposit
       ↓
3. Contract emits a deposit event
       ↓
4. AI agent detects the event and deploys funds to best protocol (e.g. Blend)
       ↓
5. Yield accumulates 24/7 — agent rebalances hourly if better opportunities exist
       ↓
6. User requests withdrawal anytime → agent pulls funds → sends back in seconds
Three Investment Strategies

Conservative — Stablecoin lending on Blend. Low risk, steady 3–6% APY.
Balanced — Mix of lending + DEX liquidity provision. Medium risk, 6–10% APY.
Growth — Aggressive multi-protocol deployment. Higher risk, 10–15% APY.


Tech Stack
Smart Contracts

Language: Rust (Soroban SDK 21.0.0)
Standard: ERC-4626 inspired vault architecture
Network: Stellar Mainnet / Testnet
Security: OpenZeppelin-equivalent patterns (ReentrancyGuard, Pausable, Access Control)

Backend / AI Agent

Runtime: Node.js or Python
Stellar SDK: @stellar/stellar-sdk
AI: Claude API / OpenAI for natural language intent parsing
Database: PostgreSQL / Supabase for user position tracking
Queue: Bull / Redis for reliable transaction processing

Frontend

Framework: Next.js 15
Blockchain: Stellar SDK + Freighter wallet integration
Styling: Tailwind CSS
Charts: Recharts for portfolio analytics

Integrations

Yield Protocols: Blend Protocol (lending), Stellar DEX (liquidity)
Price Feeds: Stellar anchor price feeds


Project Structure
neurowealth/
├── contracts/                  # Soroban smart contracts (Rust)
│   └── vault/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs          # Core vault contract
├── agent/                      # AI agent backend
│   ├── index.ts                # Agent entry point
│   ├── stellar.ts              # Stellar transaction helpers
│   ├── strategies/             # Yield strategy logic
│   │   ├── conservative.ts
│   │   ├── balanced.ts
│   │   └── growth.ts
│   ├── protocols/              # DeFi protocol integrations
│   │   └── blend.ts
│   └── nlp/                    # Natural language intent parsing
│       └── parser.ts
├── frontend/                   # Next.js web app
│   ├── app/
│   ├── components/
│   └── lib/
├── whatsapp/                   # WhatsApp bot handler
│   ├── webhook.ts
│   └── responses.ts
├── scripts/                    # Deployment and utility scripts
│   ├── deploy.sh
│   └── initialize.sh
└── README.md

Getting Started
Prerequisites
bash# Install Rust and the wasm target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt

# Install Node.js dependencies (for agent and frontend)
npm install
Environment Variables
Create a .env file in the root:
env# Stellar
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
AGENT_SECRET_KEY=your_agent_stellar_secret_key

# Contract
VAULT_CONTRACT_ID=your_deployed_contract_id
USDC_TOKEN_ADDRESS=usdc_token_address_on_stellar

# AI
ANTHROPIC_API_KEY=your_anthropic_api_key

# WhatsApp
TWILIO_ACCOUNT_SID=your_twilio_sid
TWILIO_AUTH_TOKEN=your_twilio_token
WHATSAPP_FROM=whatsapp:+14155238886

# Database
DATABASE_URL=postgresql://user:password@localhost:5432/neurowealth
Build and Deploy the Contract
bash# Build the Soroban vault contract
cd contracts
stellar contract build

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/neurowealth_vault.wasm \
  --source deployer \
  --network testnet

# Initialize the contract with your agent address and USDC token
stellar contract invoke \
  --id YOUR_CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --agent YOUR_AGENT_ADDRESS \
  --usdc_token USDC_TOKEN_ADDRESS
Run the AI Agent
bashcd agent
npm install
npm run dev        # development
npm run start      # production
Run the Frontend
bashcd frontend
npm install
npm run dev        # http://localhost:3000

Smart Contract
The core Soroban vault contract handles all on-chain fund management.
Key Functions
FunctionWho Can CallDescriptioninitializeOwner (once)Set agent address and USDC tokendepositAny verified userDeposit USDC into the vaultwithdrawUser (their own funds)Withdraw USDC back to walletrebalanceAI Agent onlyMove funds between yield strategiesget_balanceAnyoneRead a user's current balanceget_total_depositsAnyoneRead total vault TVL
Security Model

Users can only withdraw their own funds — enforced at the contract level via user.require_auth()
Only the designated AI agent keypair can call rebalance — no other address can move funds between protocols
Minimum deposit: 1 USDC. Maximum per user: 10,000 USDC (configurable)
Emergency pause functionality available to contract owner


AI Agent
The agent runs as a persistent background service with two main loops.
Decision Loop (runs every hour)
1. Fetch current APY from all integrated protocols (Blend, DEX pools)
2. Compare against each user's current deployed strategy
3. If a better opportunity exists (> 0.5% improvement), rebalance
4. Submit rebalance transaction to vault contract
5. Log results to database
Intent Parser (real-time, event-driven)
User message: "deposit 50 USDC into balanced strategy"
       ↓
AI parses intent: { action: "deposit", amount: 50, strategy: "balanced" }
       ↓
Agent builds Stellar transaction
       ↓
Returns confirmation: "Deposited 50 USDC. Earning ~8.2% APY in Balanced strategy."
Supported User Intents

deposit [amount] [optional: strategy]
withdraw [amount or "all"]
balance / how much do I have
earnings / how much have I made
switch to [conservative/balanced/growth]
what is my APY


WhatsApp Integration
NeuroWealth is designed to be fully operable through WhatsApp, making it accessible to anyone with a smartphone — no wallet app or browser extension needed.
User Flow
1. User sends "hi" to NeuroWealth WhatsApp number
2. Bot introduces itself and asks for phone number verification (OTP)
3. OTP verified → agent creates a Stellar keypair for this user (custodial)
4. User can now deposit, withdraw, and check balance entirely through chat
5. Funds are secured in the Soroban vault contract under their wallet address
Setting Up the Webhook
bash# Your webhook endpoint receives WhatsApp messages
POST /api/whatsapp/webhook

# Register your webhook URL with Twilio
# ngrok http 3000  ← for local testing
Example Conversation
User:    deposit 100 USDC
Agent:   Got it! Depositing 100 USDC into your Balanced strategy.
         This should take about 5 seconds on Stellar... ✅ Done!
         You're now earning ~8.4% APY. I'll optimize automatically.

User:    what's my balance?
Agent:   💰 Your NeuroWealth Portfolio
         Balance: 100.23 USDC
         Earnings today: +$0.23
         Current APY: 8.4%
         Strategy: Balanced

User:    withdraw everything
Agent:   Withdrawing 100.23 USDC... ✅ Done!
         Funds sent to your wallet. Arrived in 4 seconds.

Deployment
Testnet
bash# Deploy everything to Stellar testnet
./scripts/deploy.sh --network testnet
Mainnet
bash# Ensure all tests pass first
cargo test
npm test

# Deploy to mainnet
./scripts/deploy.sh --network mainnet
Infrastructure (Recommended)

Agent: Railway, Render, or a VPS (needs to run 24/7)
Frontend: Vercel
Database: Supabase (managed PostgreSQL)
Webhook: Same server as agent, or a separate serverless function


Roadmap
Phase 1 — Foundation (Current)

 Soroban vault contract (deposit, withdraw, rebalance)
 Basic AI agent with Blend protocol integration
 Natural language intent parsing
 Web frontend with portfolio dashboard
 WhatsApp bot MVP

Phase 2 — Intelligence

 Multi-protocol yield aggregation (Blend + DEX liquidity pools)
 Strategy backtesting and risk scoring
 Personalized risk profiles per user
 Earnings history and projection charts

Phase 3 — Scale

 Real-world asset (RWA) yield strategies
 Cross-chain bridging (Stellar ↔ Ethereum via Axelar)
 Social trading — follow top-performing AI strategies
 NeuroWealth token for governance and fee sharing


Contributing
Contributions are welcome. Please open an issue first to discuss what you'd like to change.
bash# Fork the repo, then:
git checkout -b feature/your-feature-name
git commit -m "feat: add your feature"
git push origin feature/your-feature-name
# Open a Pull Request
Please make sure to run cargo test and npm test before submitting.
