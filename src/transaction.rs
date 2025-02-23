// src/transaction.rs

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, Bytes};

#[serde_as]
#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    /// Public key or address of the sender.
    pub sender: [u8; 32],

    /// Optional recipient—could be None if it's, e.g., a special system transaction.
    pub recipient: Option<[u8; 32]>,

    /// Optional amount—for payment transfers, staking, etc.
    pub amount: Option<u128>,

    /// Custom data or instructions.
    pub payload: Vec<u8>,

    /// Optional signature. Could store a 64-byte signature for Ed25519, etc.
    #[serde_as(as = "Option<Bytes>")]
    pub signature: Option<[u8; 64]>,
}
