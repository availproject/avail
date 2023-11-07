// SPDX-License-Identifier: Apache-2.0
// Modified from https://github.com/QEDK/solidity-misc/blob/master/contracts/Merkle.sol
pragma solidity ^0.8.21;

import "@openzeppelin/contracts/access/Ownable.sol";
// or for foundry:
// import "openzeppelin-contracts/contracts/access/Ownable.sol";

interface IDataAvailabilityRouter {
    function roots(uint32 blockNumber) external view returns (bytes32 root);
}

contract ValidiumContract is Ownable {
    IDataAvailabilityRouter private router;

    function setRouter(
        IDataAvailabilityRouter _router
    ) public virtual onlyOwner {
        router = _router;
    }

    function checkDataRootMembership(
        uint32 blockNumber,
        bytes32[] calldata proof,
        uint256 width, // number of leaves
        uint256 index,
        bytes32 leaf
    ) public view virtual returns (bool isMember) {
        bytes32 rootHash = router.roots(blockNumber);
        // if root hash is 0, block does not have a root (yet)
        require(rootHash != bytes32(0), "INVALID_ROOT");
        assembly ("memory-safe") {
            if proof.length {
                let end := add(proof.offset, shl(5, proof.length))
                let i := proof.offset

                for {} 1 {} {
                    let leafSlot := shl(5, and(0x1, index))
                    if eq(add(index, 1), width) {
                        leafSlot := 0x20
                    }
                    mstore(leafSlot, leaf)
                    mstore(xor(leafSlot, 32), calldataload(i))
                    leaf := keccak256(0, 64)
                    index := shr(1, index)
                    i := add(i, 32)
                    width := add(shr(1, sub(width, 1)), 1)
                    if iszero(lt(i, end)) {
                        break
                    }
                }
            }
            // checks if the calculated root matches the expected root
            isMember := eq(leaf, rootHash)
        }
    }
}
