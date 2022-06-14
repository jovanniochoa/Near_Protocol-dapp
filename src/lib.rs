#![allow(non_snake_case)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::{current_account_id, is_valid_account_id};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use std::collections::HashMap;

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

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct Contract {
    genesisLockOnce: bool,
    genesisStartOnce: bool,

    adminAddress: AccountId,    // address of the admin
    operatorAddress: AccountId, // address of the operator

    bufferSeconds: u128, // number of seconds for valid execution of a prediction round
    intervalSeconds: u128, // interval in seconds between two prediction rounds
    intervalBlocks: u128, // interval in seconds between two prediction rounds
    bufferBlocks: u128,  // number of seconds for valid execution of a prediction round

    minBetAmount: u128,   // minimum betting amount (denominated in wei)
    treasuryFee: u128,    // treasury rate (e.g. 200 = 2%, 150 = 1.50%)
    treasuryAmount: u128, // treasury amount that was not claimed

    currentEpoch: u128, // current epoch for prediction round

    oracleLatestRoundId: u128,   // converted from uint80 (Chainlink)
    oracleUpdateAllowance: u128, // seconds

    MAX_TREASURY_FEE: u128, // 10%

    TOTAL_RATE: u128,   // 100%
    rewardRate: u128,   // 90%
    treasuryRate: u128, // 10%

    rounds: HashMap<String, u128>,
    userRounds: HashMap<String, u128>,
    ledger: HashMap<String, u128>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            genesisLockOnce: false,
            genesisStartOnce: false,
            adminAddress: env::signer_account_id(),
            operatorAddress: env::predecessor_account_id(),
            bufferSeconds: 0,
            intervalSeconds: 0,
            treasuryFee: 0,
            treasuryAmount: 0,
            currentEpoch: 0,
            oracleLatestRoundId: 0,
            oracleUpdateAllowance: 0,
            MAX_TREASURY_FEE: 1000,
            intervalBlocks: 0,
            bufferBlocks: 0,
            TOTAL_RATE: 100,
            rewardRate: 90,   // 90%
            treasuryRate: 10, // 10%
            minBetAmount: 1,
            ledger: HashMap::new(),
            rounds: HashMap::new(),
            userRounds: HashMap::new(),
        }
    }

    // fn isAdmin(&self) -> bool {
    //   return self.adminAddress == env::signer_account_id()
    // }

    // fn isOperator(&self) -> bool {
    //   return self.operatorAddress == current_account_id()
    // }

    // pub fn setIntervalBlocks(&mut self ,_intervalBlocks: u128){
    //   require!(self.isAdmin());
    //   self.intervalBlocks = _intervalBlocks;
    // }

    // pub fn setBufferBlocks(&mut self, _bufferBlocks: u128){
    //   require!(self.isOperator());
    //   require!(_bufferBlocks <= self.intervalBlocks, "Cannot be more than intervalBlocks");
    //   self.bufferBlocks = _bufferBlocks;
    // }

    // /*
    // pub fn setOracle(&mut self, _oracle: AccountId){
    //   require!(self.isAdmin());
    //   require!(is_valid_account_id(_oracle));
    //   oracle = AggregatorV3Interface(_oracle);
    // }
    // */
    // pub fn setOracleUpdateAllowance(&mut self, _oracleUpdateAllowance: u128){
    //   require!(self.isAdmin());
    //   self.oracleUpdateAllowance = _oracleUpdateAllowance;
    // }

    // pub fn setRewardRate(&mut self, _rewardRate: u128){
    //   require!(self.isAdmin());
    //   require!(_rewardRate <= self.TOTAL_RATE, "rewardRate cannot be more than 100%");
    //   self.rewardRate = _rewardRate;
    //   self.treasuryRate = self.TOTAL_RATE.sub(_rewardRate);
    // }

    // pub fn setTreasuryRate(&mut self, _treasuryRate: u128){
    //   require!(self.isAdmin());
    //   require!(_treasuryRate <= self.TOTAL_RATE, "treasuryRate cannot be more than 100%");
    //   self.rewardRate = self.TOTAL_RATE.sub(_treasuryRate);
    //   self.treasuryRate = _treasuryRate;
    // }

    // pub fn setMinBetAmount(&mut self, _minBetAmount: u128){
    //   require!(self.isAdmin());
    //   self.minBetAmount = _minBetAmount;
    // }

    // pub fn genesisStartRound(&mut self){
    //   require!(self.isOperator());
    //   require!(!self.genesisStartOnce, "Can only run genesisStartRound once");

    //   self.currentEpoch += 1;
    //   _startRound(self.currentEpoch);
    //   self.genesisStartOnce = true;
    // }

    // pub fn genesisLockRound(&mut self, currentRoundId: u128, currentPrice: u128){
    //   require!(self.isOperator());
    //   require!(self.genesisStartOnce, "Can only run after genesisStartRound is triggered");
    //   require!(!self.genesisLockOnce, "Can only run genesisLockRound once");

    //   _getPriceFromOracle(currentRoundId, currentPrice);
    //   _safeLockRound(self.currentEpoch, currentRoundId, currentPrice);

    //   self.currentEpoch = self.currentEpoch + 1;
    //   _startRound(self.currentEpoch);
    //   self.genesisLockOnce = true;

    // pub fn _startRound(epoch: u128){
    //   Round storage round = rounds[epoch];
    //   round.startBlock = block.number;
    //   round.lockBlock = block.number.add(intervalBlocks);
    //   round.closeBlock = block.number.add(intervalBlocks * 2);
    //   round.epoch = epoch;
    //   round.totalAmount = 0;

    //   StartRound(epoch);
    // }
}
