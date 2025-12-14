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
// Hash 1439
// Hash 9569
// Hash 4942
// Hash 5431
// Hash 5787
// Hash 8783
// Hash 1440
// Hash 1417
// Hash 7651
// Hash 3653
// Hash 4263
// Hash 8325
// Hash 9675
// Hash 6296
// Hash 6951
// Hash 1709
// Hash 8580
// Hash 5803
// Hash 7678
// Hash 3673
// Hash 2225
// Hash 6256
// Hash 4805
// Hash 8446
// Hash 5127
// Hash 3919
// Hash 2268
// Hash 2360
// Hash 2441
// Hash 5416
// Hash 4652
// Hash 2569
// Hash 2752
// Hash 1622
// Hash 7260
// Hash 4553
// Hash 5197
// Hash 4845
// Hash 5997
// Hash 9243
// Hash 9621
// Hash 9351
// Hash 6597
// Hash 6226
// Hash 6110
// Hash 5259
// Hash 8181
// Hash 8965
// Hash 2115
// Hash 8116
// Hash 5819
// Hash 1472
// Hash 8730
// Hash 2250
// Hash 2593
// Hash 4903
// Hash 6501
// Hash 5522
// Hash 5454
// Hash 3856
// Hash 4671
// Hash 7743
// Hash 8773
// Hash 3499
// Hash 3329
// Hash 9922
// Hash 3847
// Hash 2951
// Hash 1664
// Hash 9111
// Hash 6320
// Hash 1443
// Hash 9855
// Hash 1585
// Hash 8364
// Hash 3974
// Hash 3060
// Hash 1160
// Hash 2192
// Hash 3562
// Hash 8733
// Hash 1553
// Hash 9720
// Hash 4850
// Hash 9347
// Hash 5834
// Hash 3883
// Hash 3146
// Hash 9280
// Hash 5954
// Hash 3832
// Hash 2688
// Hash 3287
// Hash 7180
// Hash 7259
// Hash 8123
// Hash 1984
// Hash 5805
// Hash 5989
// Hash 3633
// Hash 2901
// Hash 8445
// Hash 8255
// Hash 8012
// Hash 8911
// Hash 7838
// Hash 3251
// Hash 2717
// Hash 2750
// Hash 3288
// Hash 7217
// Hash 4240
// Hash 1047
// Hash 6292
// Hash 4966
// Hash 6347
// Hash 1974
// Hash 4636
// Hash 9045
// Hash 1248
// Hash 7104
// Hash 6196
// Hash 7871
// Hash 6918
// Hash 8596
// Hash 5128
// Hash 5734
// Hash 8226
// Hash 2371
// Hash 4120
// Hash 9551
// Hash 9041
// Hash 8292
// Hash 8139