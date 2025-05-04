use sha2::{Sha256, Digest};
use hex::encode;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub leaves: Vec<String>,
    pub levels: Vec<Vec<String>>,
}

impl MerkleTree {
    pub fn new(data: Vec<String>) -> Self {
        let mut leaves = data.into_iter()
            .map(|d| hash(&d))
            .collect::<Vec<String>>();

        let mut levels = vec![leaves.clone()];
        
        while leaves.len() > 1 {
            leaves = build_level(leaves);
            levels.push(leaves.clone());
        }

        MerkleTree { leaves: levels[0].clone(), levels }
    }

    pub fn root(&self) -> String {
        self.levels.last().unwrap()[0].clone()
    }

    pub fn generate_proof(&self, index: usize) -> Vec<(String, bool)> {
        let mut proof = vec![];
        let mut idx = index;

        for level in &self.levels[..self.levels.len() - 1] {
            let is_right = idx % 2 == 1;
            let pair_index = if is_right { idx - 1 } else { idx + 1 };

            if pair_index < level.len() {
                proof.push((level[pair_index].clone(), is_right));
            }

            idx /= 2;
        }

        proof
    }

    pub fn verify_proof(leaf: &str, proof: Vec<(String, bool)>, root: &str) -> bool {
        let mut hash = hash(leaf);

        for (sibling, is_right) in proof {
            hash = if is_right {
                hash_pair(&sibling, &hash)
            } else {
                hash_pair(&hash, &sibling)
            };
        }

        hash == root
    }
}

fn build_level(prev_level: Vec<String>) -> Vec<String> {
    prev_level.chunks(2)
        .map(|chunk| {
            if chunk.len() == 2 {
                hash_pair(&chunk[0], &chunk[1])
            } else {
                chunk[0].clone()
            }
        })
        .collect()
}

fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    encode(hasher.finalize())
}

fn hash_pair(a: &str, b: &str) -> String {
    hash(&(a.to_owned() + b))
}

fn main() {
    let data = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    let tree = MerkleTree::new(data.clone());
    println!("Root: {}", tree.root());

    let leaf_index = 2;
    let proof = tree.generate_proof(leaf_index);
    println!("Proof for '{}': {:?}", data[leaf_index], proof);

    let is_valid = MerkleTree::verify_proof(&data[leaf_index], proof, &tree.root());
    println!("Proof valid? {}", is_valid);
}
