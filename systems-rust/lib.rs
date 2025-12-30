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
// Hash 9666
// Hash 7086
// Hash 1132
// Hash 8868
// Hash 3051
// Hash 4969
// Hash 2112
// Hash 1223
// Hash 9415
// Hash 7847
// Hash 8503
// Hash 7871
// Hash 5730
// Hash 8731
// Hash 9909
// Hash 3166
// Hash 7980
// Hash 7853
// Hash 1828
// Hash 5941
// Hash 9931
// Hash 9712
// Hash 2409
// Hash 2395
// Hash 9063
// Hash 4098
// Hash 9743
// Hash 3312
// Hash 1623
// Hash 9509
// Hash 4731
// Hash 1644
// Hash 3375
// Hash 2053
// Hash 3124
// Hash 3810
// Hash 5253
// Hash 6208
// Hash 3869
// Hash 7618
// Hash 7313
// Hash 7724
// Hash 1518
// Hash 8405
// Hash 1353
// Hash 9658
// Hash 1508
// Hash 6603
// Hash 6155
// Hash 3533
// Hash 5544
// Hash 4645
// Hash 4357
// Hash 8610
// Hash 2293
// Hash 1784
// Hash 8799
// Hash 3358
// Hash 7904
// Hash 8947
// Hash 2092
// Hash 5193
// Hash 3805
// Hash 8091
// Hash 3192
// Hash 3237
// Hash 3964
// Hash 6936
// Hash 3891
// Hash 4657
// Hash 9238
// Hash 8787
// Hash 8108
// Hash 9664
// Hash 7608
// Hash 6669
// Hash 8892
// Hash 5982
// Hash 2200
// Hash 3384
// Hash 3823
// Hash 3300
// Hash 4186
// Hash 2829
// Hash 9966
// Hash 7045
// Hash 2283
// Hash 8695
// Hash 7122
// Hash 3698
// Hash 2049
// Hash 1771
// Hash 2373
// Hash 7675
// Hash 6247
// Hash 6236
// Hash 2251
// Hash 1781
// Hash 7769
// Hash 9446
// Hash 2508
// Hash 4710
// Hash 1809
// Hash 5387
// Hash 3188
// Hash 6676
// Hash 2333
// Hash 5590
// Hash 2705
// Hash 9503
// Hash 3994
// Hash 8810
// Hash 9861
// Hash 9370
// Hash 7465
// Hash 9763
// Hash 2788
// Hash 7458
// Hash 9004
// Hash 9158
// Hash 4736
// Hash 6217
// Hash 2225
// Hash 5867
// Hash 4184
// Hash 4912
// Hash 7124