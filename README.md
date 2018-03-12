# Merkle Tree

Merkle Tree, if I got it right from my research, is a special type of tree,
in which each node consists of either
- its own value and a hash of this value;
- two child nodes carrying their own hashes, and the node's hash, derived from children's hash by some algorithms.
The container is aimed to make it easy to verify the integrity and immutability of data within it.
Two sets of data are considered to be identical if two merkle trees each built on top of corresponding set have equal root hashes (merkle root).
Two sets of data are considered non-identical otherwise.
If the data has been changed, the newly built merkle tree on top of it shall have a different merkle root.
