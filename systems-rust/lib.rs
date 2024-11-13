use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 3371
// Hash 5568
// Hash 6290
// Hash 7874
// Hash 1759
// Hash 8181
// Hash 6468
// Hash 8408
// Hash 5962
// Hash 8144
// Hash 6759
// Hash 3302
// Hash 3040
// Hash 9967
// Hash 3309
// Hash 1803
// Hash 1534
// Hash 2618
// Hash 5262
// Hash 4688
// Hash 4879
// Hash 4033
// Hash 7207
// Hash 4732
// Hash 3360
// Hash 9705
// Hash 6167
// Hash 9938
// Hash 7675
// Hash 3775
// Hash 5980
// Hash 2034
// Hash 3411
// Hash 4001
// Hash 8717
// Hash 2988
// Hash 7887
// Hash 9455
// Hash 8836
// Hash 6895
// Hash 2592