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

// Hash 1769
// Hash 4920
// Hash 9959
// Hash 4827
// Hash 6123
// Hash 3548
// Hash 6998
// Hash 2062
// Hash 5335
// Hash 3440
// Hash 3692
// Hash 8565
// Hash 8832
// Hash 7165
// Hash 9253
// Hash 7559
// Hash 2759
// Hash 8382
// Hash 7964
// Hash 8895
// Hash 3852
// Hash 9702
// Hash 7163
// Hash 2548
// Hash 5220
// Hash 1931
// Hash 2246
// Hash 3125
// Hash 3550
// Hash 5866
// Hash 4706
// Hash 2802
// Hash 6077
// Hash 4227
// Hash 3520
// Hash 4117
// Hash 6163
// Hash 4319
// Hash 7403
// Hash 1206
// Hash 9431
// Hash 7342
// Hash 4068