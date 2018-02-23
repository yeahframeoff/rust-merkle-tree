use std::option::Option;
use std::boxed::Box;

type TreeValue = [i8; 32];
type Hash = String;

enum MerkleNodePayload {
    Leaf {value: Option<TreeValue>},
    Node (Box<MerkleNode>, Box<MerkleNode>)
}

struct MerkleNode {
    hash: Hash,
    payload: MerkleNodePayload
}

struct MerkleTree {
    root: MerkleNode
}
