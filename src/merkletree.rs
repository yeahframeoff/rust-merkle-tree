use std::boxed::Box;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use itertools::Itertools;

enum MerkleNodePayload<T: ToString> {
    Leaf (T),
    Node (Box<MerkleNode<T>>, Box<MerkleNode<T>>)
}

use self::MerkleNodePayload::{Leaf, Node};

struct MerkleNode<T: ToString> {
    hash: String,
    payload: MerkleNodePayload<T>
}

pub struct MerkleTree<T: ToString> {
    root: MerkleNode<T>,
    leaves: Vec<T>,
}

fn hash(val: String) -> String {
    let mut hasher1 = Sha256::new();
    let mut hasher2 = Sha256::new();

    hasher1.input_str(&val);
    let hash1 = hasher1.result_str();
    
    hasher2.input_str(&hash1);
    let hash2 = hasher2.result_str();

    hash2
}

fn new_leaf<T>(val: T) -> MerkleNode<T>
where T: ToString {
    MerkleNode {
        hash: hash(val.to_string()),
        payload: Leaf(val)
    }
}


fn combine(s1: &String, s2: &String) -> String {
    format!("{}{}", s1, s2)
}


fn new_node<T>(left: MerkleNode<T>, right: MerkleNode<T>) -> MerkleNode<T>
where T: ToString {
    let concatenated = combine(&left.hash, &right.hash);
    MerkleNode {
        hash: hash(concatenated),
        payload: Node(Box::new(left), Box::new(right))
    }
}


fn build_layer<T>(items: Vec<MerkleNode<T>>) -> Vec<MerkleNode<T>>
where T: ToString {
    
    let new_layer = items.into_iter().batching(|it| {
        match it.next() {
            Some(left) => match it.next() {
                Some(right) => Some(new_node(left, right)),
                None => Some(left)
            },
            None => None
        }
    });

    new_layer.collect::<Vec<_>>()
}


impl<T: ToString> MerkleTree<T> {
    pub fn from_leaves<II>(items: II) -> MerkleTree<II::Item>
    where II: IntoIterator,
          II::Item: ToString + Clone
    {
        let leaves = items.into_iter().collect::<Vec<_>>();

        let mut layer: Vec<_> = leaves.iter().cloned().map(new_leaf).collect();

        while layer.len() != 1 {
            layer = build_layer(layer);
        }

        match layer.pop() { 
            Some(root) => MerkleTree { root, leaves },
            None => panic!("You should not have built an empty tree")
        }
    }

    pub fn get_root(&self) -> String {
        self.root.hash.clone()
    }
}
