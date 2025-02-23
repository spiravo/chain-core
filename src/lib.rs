pub mod block;
pub mod transaction;
pub mod crypto;
pub mod codec;
pub mod error;

pub use block::Block;
pub use transaction::Transaction;
pub use crypto::{hashing, signature};

#[cfg(test)]
mod tests {
    use super::*;
    use parity_scale_codec::{Encode, Decode};

    #[test]
    fn test_block_encode_decode() {
        let block = block::Block {
            header: block::Header {
                parent_hash: [0u8; 32],
                number: 42,
                state_root: [1u8; 32],
                extrinsics_root: [2u8; 32],
            },
            transactions: vec![transaction::Transaction {
                sender: [9u8; 32],
                recipient: Some([8u8; 32]),
                amount: Some(100),
                payload: vec![1, 2, 3],
                signature: Some([7u8; 64]),
            }],
        };

        // SCALE encode + decode
        let encoded = block.encode();
        let decoded = block::Block::decode(&mut &encoded[..])
            .expect("decode should work");
        assert_eq!(block, decoded, "Block should round-trip correctly");
    }
}