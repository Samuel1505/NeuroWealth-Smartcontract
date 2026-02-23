#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, 
    Address, Env, Symbol, symbol_short
};

// ---- Storage Keys ----
#[contracttype]
pub enum DataKey {
    Balance(Address),   // user -> usdc balance
    TotalDeposits,      // total USDC in vault
    Agent,              // authorized AI agent address
    UsdcToken,          // USDC token contract address
}

// ---- Events ----
#[contracttype]
pub struct DepositEvent {
    pub user: Address,
    pub amount: i128,
}

#[contracttype]
pub struct WithdrawEvent {
    pub user: Address,
    pub amount: i128,
}

// ---- Contract ----
#[contract]
pub struct NeuroWealthVault;

#[contractimpl]
impl NeuroWealthVault {

    // Called once when deploying the contract
    pub fn initialize(env: Env, agent: Address, usdc_token: Address) {
        // Make sure this can only be called once
        if env.storage().instance().has(&DataKey::Agent) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Agent, &agent);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&DataKey::TotalDeposits, &0_i128);
    }

    // User deposits USDC into the vault
    pub fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth(); // user must sign this transaction

        assert!(amount > 0, "Amount must be positive");
        assert!(amount >= 1_000_000, "Minimum deposit is 1 USDC"); // USDC has 7 decimals

        // Transfer USDC from user to this contract
        let usdc_token: Address = env.storage().instance()
            .get(&DataKey::UsdcToken).unwrap();
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(&user, &env.current_contract_address(), &amount);

        // Update user balance
        let current_balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);
        env.storage().persistent()
            .set(&DataKey::Balance(user.clone()), &(current_balance + amount));

        // Update total deposits
        let total: i128 = env.storage().instance()
            .get(&DataKey::TotalDeposits).unwrap_or(0);
        env.storage().instance()
            .set(&DataKey::TotalDeposits, &(total + amount));

        // Emit deposit event (your AI agent listens for this)
        env.events().publish(
            (symbol_short!("deposit"),),
            DepositEvent { user, amount }
        );
    }

    // User withdraws USDC from the vault
    pub fn withdraw(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);

        assert!(amount > 0, "Amount must be positive");
        assert!(balance >= amount, "Insufficient balance");

        // Update user balance
        env.storage().persistent()
            .set(&DataKey::Balance(user.clone()), &(balance - amount));

        // Update total deposits
        let total: i128 = env.storage().instance()
            .get(&DataKey::TotalDeposits).unwrap_or(0);
        env.storage().instance()
            .set(&DataKey::TotalDeposits, &(total - amount));

        // Transfer USDC back to user
        let usdc_token: Address = env.storage().instance()
            .get(&DataKey::UsdcToken).unwrap();
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(&env.current_contract_address(), &user, &amount);

        // Emit withdraw event
        env.events().publish(
            (symbol_short!("withdraw"),),
            WithdrawEvent { user, amount }
        );
    }

    // Only the AI agent can call this to rebalance funds
    pub fn rebalance(env: Env, strategy: Symbol) {
        let agent: Address = env.storage().instance()
            .get(&DataKey::Agent).unwrap();
        agent.require_auth(); // ONLY the agent can rebalance

        // In Phase 2 you add Blend protocol calls here
        // For now just emit an event
        env.events().publish(
            (symbol_short!("rebalance"),),
            strategy
        );
    }

    // ---- Read Functions ----

    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    pub fn get_total_deposits(env: Env) -> i128 {
        env.storage().instance()
            .get(&DataKey::TotalDeposits)
            .unwrap_or(0)
    }

    pub fn get_agent(env: Env) -> Address {
        env.storage().instance()
            .get(&DataKey::Agent)
            .unwrap()
    }
}

// ---- Tests ----
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_deposit_and_withdraw() {
        let env = Env::default();
        env.mock_all_auths(); // skip signature checks in tests

        let contract_id = env.register_contract(None, NeuroWealthVault);
        let client = NeuroWealthVaultClient::new(&env, &contract_id);

        let agent = Address::generate(&env);
        let user = Address::generate(&env);
        
        // Note: in real tests you'd deploy a mock USDC token too
        // This is simplified for clarity

        assert_eq!(client.get_balance(&user), 0);
    }
}