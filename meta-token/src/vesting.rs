use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    serde::{Deserialize, Serialize},
};

use crate::util::*;

/// Contains information about vesting schedule.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(feature = "test", derive(Clone, Debug))]
pub struct VestingRecord {
    /// amount locked in  the vesting schedule.
    /// before transferring, vesting is checked and
    /// if we're before linear_start_timestamp or locked_until_timestamp, locked_amount = amount
    /// else if we're past the linear_end_timestamp, vesting is removed
    /// else we're past the linear_start_timestamp and before linear_end_timestamp, a linear locked_amount is computed
    pub amount: u128,
    /// Absolute timestamp until the amount is locked in full. This field allows special linear release schedules
    /// for example 50% at a certain date (locked_until_timestamp+1) and a linear release after that, can be arranged
    /// by making locked_until_timestamp to sit between linear_start_timestamp and linear_end_timestamp
    pub locked_until_timestamp: TimestampNano,
    /// The timestamp in nanosecond when linear release starts
    pub linear_start_timestamp: TimestampNano,
    /// The remaining tokens will be released linearly until linear_end_timestamp.
    pub linear_end_timestamp: TimestampNano,
}

#[derive(Deserialize, Serialize)]
pub struct VestingRecordJSON {
    pub amount: U128String,
    pub locked_until_timestamp: U64String,
    pub linear_start_timestamp: U64String,
    pub linear_end_timestamp: U64String,
}

impl VestingRecord {
    pub fn new(
        amount: u128,
        locked_until_timestamp: TimestampNano,
        linear_start_timestamp: TimestampNano,
        linear_end_timestamp: TimestampNano,
    ) -> Self {
        assert!(amount > 0, "vesting amount must be > 0");
        assert!(
            linear_start_timestamp <= linear_end_timestamp,
            "vesting: start > end"
        );
        assert!(
            locked_until_timestamp < linear_end_timestamp,
            "vesting: locked_until_timestamp >= end"
        );
        Self {
            amount,
            locked_until_timestamp,
            linear_start_timestamp,
            linear_end_timestamp,
        }
    }

    /// Get the amount of tokens that are locked in this account due to vesting or release schedule.
    pub fn compute_amount_locked(&self) -> u128 {
        let block_timestamp = env::block_timestamp();

        return if block_timestamp < self.linear_start_timestamp || block_timestamp < self.locked_until_timestamp {
            // Before the start or before the locked_until date, all is locked
            self.amount
        } else if block_timestamp >= self.linear_end_timestamp {
            // After linear_end_timestamp none is locked
            0
        } else {
            // compute time-left cannot overflow since block_timestamp < linear_end_timestamp
            let time_left = self.linear_end_timestamp - block_timestamp;
            // The total time is positive. Checked at the contract initialization.
            let total_time = self.linear_end_timestamp - self.linear_start_timestamp;
            // locked amount is linearly reduced until time_left = 0 (linear_end_timestamp)
            fraction_of(self.amount, time_left as u128, total_time as u128)
        };
    }
}
