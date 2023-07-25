//SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";

contract DataAvailabilityRouter {
    mapping(uint32 => bytes32) public roots;
}

contract ValidiumContract is Ownable {
    DataAvailabilityRouter private router;

    function setRouter(
        address _router
    ) public onlyOwner {
        router = DataAvailabilityRouter(_router);
    }

    function getDataRoot(
        uint32 blockNumber
    ) public view returns (bytes32) {
        return router.roots(blockNumber);
    }

    function checkDataRootMembership(
        uint32 blockNumber,
        bytes32[] calldata proof,
        uint256 numberOfLeaves,
        uint256 index,
        bytes32 leaf
    ) public view returns (bool isMember) {
        // if the proof is of size n, the tree height will be n+1
        // in a tree of height n+1, max possible leaves are 2^n
        require(index < numberOfLeaves, "INVALID_LEAF_INDEX");
        // refuse to accept padded leaves as proof
        require(leaf != bytes32(0), "INVALID_LEAF");

        bytes32 rootHash = getDataRoot(blockNumber);
        assembly ("memory-safe") {
            if proof.length {
                let end := add(proof.offset, shl(5, proof.length))
                let i := proof.offset
                let width := numberOfLeaves

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
            isMember := eq(leaf, rootHash)

        }
        return isMember;
    }
}

