use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize, PartialEq, Eq)]
pub struct Header {
    pub parent_hash: [u8; 32],
    pub number: u64,
    pub state_root: [u8; 32],
    pub extrinsics_root: [u8; 32],
    // Additional fields as needed
}

#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<crate::transaction::Transaction>,
}
