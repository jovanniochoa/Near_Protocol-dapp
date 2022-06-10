#![allow(non_snake_case)]
use near_sdk::serde_json::Map;
use near_sdk::{BorshStorageKey, near_bindgen, PanicOnDefault, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string;
 
#[derive(BorshStorageKey, BorshSerialize)]
enum Position {
 Bull,
 Bear,
}
 
#[derive(Debug, BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct Round {
 epoch: u128,
 startTimestamp: u128,
 lockTimestamp: u128,
 closeTimestamp: u128,
 lockPrice: i128,
 closePrice: i128,
 lockOracleId: u128,
 closeOracleId: u128,
 totalAmount: u128,
 bullAmount: u128,
 bearAmount: u128,
 rewardBaseCalAmount: u128,
 rewardAmount: u128,
 oracleCalled: bool,
}
 
struct BetInfo {
 position: Position,
 amount: u128,
 claimed: bool,
}
 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
genesisLockOnce : bool,
genesisStartOnce : bool,
 
adminAddress : i8, // address of the admin
operatorAddress : i8, // address of the operator
 
bufferSeconds : u128, // number of seconds for valid execution of a prediction round
intervalSeconds : u128, // interval in seconds between two prediction rounds
 
minBetAmount : u128, // minimum betting amount (denominated in wei)
treasuryFee : u128, // treasury rate (e.g. 200 = 2%, 150 = 1.50%)
treasuryAmount : u128, // treasury amount that was not claimed
 
currentEpoch : u128, // current epoch for prediction round
 
oracleLatestRoundId : u128, // converted from uint80 (Chainlink)
oracleUpdateAllowance : u128, // seconds
 
MAX_TREASURY_FEE : u128, // 10%
 
rounds: HashMap<String, u128>,
userRounds: HashMap<String ,u128>,
ledger: HashMap<String, u128>,
 
}
#[near_bindgen]
impl Contract {
#[init]
pub fn new() -> Self {
  Self {
    genesisLockOnce: false,
    genesisStartOnce: false,
    adminAddress: 0,
    operatorAddress: 0,
    bufferSeconds: 0,
    intervalSeconds: 0,
    minBetAmount: 0,
    treasuryFee: 0,
    treasuryAmount: 0,
    currentEpoch: 0,
    oracleLatestRoundId: 0,
    oracleUpdateAllowance: 0,
    MAX_TREASURY_FEE: 1000,
    ledger: HashMap::new(),
    rounds: HashMap::new(),
    userRounds: HashMap::new(),
   
   
  }
}
 
}
