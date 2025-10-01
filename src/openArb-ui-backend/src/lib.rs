use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use std::cell::RefCell;
use std::collections::HashMap;

// ============================================================================
// Essential Data Structures
// ============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserData {
    pub principal: Principal,
    pub wallet_connected: bool,
    pub wallet_type: Option<String>,
    pub total_profit: f64,
    pub total_trades: u32,
    pub settings: BotSettings,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BotSettings {
    pub min_profit_threshold: f64,
    pub max_trade_size: f64,
    pub slippage_tolerance: f64,
    pub auto_trading_enabled: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TradeRecord {
    pub id: String,
    pub token_pair: String,
    pub profit: f64,
    pub timestamp: u64,
    pub status: String,
}

// ============================================================================
// Storage
// ============================================================================

thread_local! {
    static USERS: RefCell<HashMap<Principal, UserData>> = RefCell::new(HashMap::new());
    static TRADES: RefCell<HashMap<String, TradeRecord>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(1);
}

// ============================================================================
// Core Functions
// ============================================================================

fn get_caller() -> Principal {
    ic_cdk::api::caller()
}

fn generate_id() -> String {
    NEXT_ID.with(|id| {
        let current = *id.borrow();
        *id.borrow_mut() = current + 1;
        format!("trade_{}", current)
    })
}

#[ic_cdk::update]
pub fn connect_wallet(wallet_type: String) -> Result<(), String> {
    let caller = get_caller();

    let user_data = UserData {
        principal: caller,
        wallet_connected: true,
        wallet_type: Some(wallet_type),
        total_profit: 0.0,
        total_trades: 0,
        settings: BotSettings {
            min_profit_threshold: 0.5,
            max_trade_size: 1000.0,
            slippage_tolerance: 1.0,
            auto_trading_enabled: false,
        },
        created_at: time(),
    };

    USERS.with(|users| {
        users.borrow_mut().insert(caller, user_data);
    });

    Ok(())
}

#[ic_cdk::update]
pub fn save_settings(settings: BotSettings) -> Result<(), String> {
    let caller = get_caller();

    USERS.with(|users| match users.borrow_mut().get_mut(&caller) {
        Some(user) => {
            user.settings = settings;
            Ok(())
        }
        None => Err("User not found".to_string()),
    })
}

#[ic_cdk::update]
pub fn record_trade(token_pair: String, profit: f64) -> Result<String, String> {
    let caller = get_caller();
    let trade_id = generate_id();

    let trade = TradeRecord {
        id: trade_id.clone(),
        token_pair,
        profit,
        timestamp: time(),
        status: "completed".to_string(),
    };

    // Store trade
    TRADES.with(|trades| {
        trades.borrow_mut().insert(trade_id.clone(), trade);
    });

    // Update user stats
    USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&caller) {
            user.total_profit += profit;
            user.total_trades += 1;
        }
    });

    Ok(trade_id)
}

#[ic_cdk::query]
pub fn get_user_data() -> Result<UserData, String> {
    let caller = get_caller();

    USERS.with(|users| {
        users
            .borrow()
            .get(&caller)
            .cloned()
            .ok_or("User not found".to_string())
    })
}

#[ic_cdk::query]
pub fn get_trade_history() -> Vec<TradeRecord> {
    // NOTE: This returns all trades, not just those for the caller.
    TRADES.with(|trades| trades.borrow().values().cloned().collect())
}

// Mock data endpoints for frontend
#[ic_cdk::query]
pub fn get_arbitrage_opportunities() -> Vec<(String, String, f64, f64)> {
    // Return mock data - frontend handles the rest
    vec![
        (
            "ICP/USDC".to_string(),
            "Sonic DEX → ICPSwap".to_string(),
            2.34,
            234.50,
        ),
        (
            "ckBTC/ICP".to_string(),
            "InfinitySwap → Sonic DEX".to_string(),
            1.87,
            187.30,
        ),
    ]
}

#[ic_cdk::query]
pub fn get_price_feed() -> Vec<(String, f64, u64)> {
    // Return mock price data
    vec![
        ("ICP".to_string(), 7.234, time()),
        ("ckBTC".to_string(), 43234.56, time()),
        ("USDC".to_string(), 1.0, time()),
    ]
}

#[ic_cdk::init]
fn init() {
    ic_cdk::println!("OpenArb Trading Bot Backend initialized");
}

// Export the Candid interface for this canister
ic_cdk::export_candid!();
