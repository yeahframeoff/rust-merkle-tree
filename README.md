# Merkle Tree (Rust)

Merkle Tree, if I got it right from my research, is a special type of tree,
in which each node consists of either
- its own value and a hash of this value;
- two child nodes carrying their own hashes, and the node's hash, derived from children's hash by some algorithms.

The container is aimed to make it easy to verify the integrity and immutability of data within it.
Two sets of data are considered to be identical if two merkle trees each built on top of corresponding set have equal root hashes (merkle root).
Two sets of data are considered non-identical otherwise.
If the data has been changed, the newly built merkle tree on top of it shall have a different merkle root.


## Implementation
In this project I have implemented a Merkle Tree data structure.
The container implements the only method - 
building essentially a merkle tree from an arbitrary [`IntoIterator`](https://doc.rust-lang.org/1.21.0/std/iter/trait.IntoIterator.html) 
of [`ToString`](https://doc.rust-lang.org/1.21.0/std/string/trait.ToString.html) items,
with latter getting a root hash value (having type `String`).

Each node of the tree is implemented struct with fields `hash: String` and `payload: MerkleNodePayload`.

A `MerkleNodePayload` is `enum` with possible variants:
- `Leaf` - essentially, a leaf, having a certain `ToString` value within it;
- `Node` - intermediate node, having links to left and right children nodes, allocated on the heap.

Hashing algorithm pseudocode:
```
hash(value) = sha256(sha256(value))
```

In order to build a tree, all the leaves are gathered as `Vec<Leaf<T>>`.
The resulting 'layer' of nodes is folded several times.
On each iteration the new layer of parent nodes is created.
If a particular layer has odd number of nodes, the last node is propagated to next level *untouched*.
The iteration repeats until the layer remains with only one node. That node becomes **the root** of the tree. 

The implementation has no methods of expanding the tree (adding new elements) but can be easily adopted to support it.
The implementation either has no methods of getting an authorization path of a node, but after adding additional fields to node structs (`Weak` references to parent node) it can work.

The example of work is demonstrated in `tests\tests.rs`.

To see tests pass run `$ cargo test`.

Here is the simplified version:

```
extern crate merkle_tree;
use merkle_tree::MerkleTree;


#[test]
fn test_equal_content_has_equal_merkle_root() {
    let content1 = vec!["A", "B", "C"];
    let content2 = vec!["A", "B", "C"];
    let tree1 = MerkleTree::<&str>::from_leaves(content1);
    let tree2 = MerkleTree::<&str>::from_leaves(content2);
    assert_eq!(tree1.get_root(), tree2.get_root());
}


#[test]
fn test_different_content_produces_different_merkle_root() {
    let content1 = vec!["A", "B", "C", "D"];

    let content2 = vec!["A", "B", "C", "E"];

    let tree1 = MerkleTree::<&str>::from_leaves(content1);
    let tree2 = MerkleTree::<&str>::from_leaves(content2);
    assert!(tree1.get_root() != tree2.get_root());
}
```
