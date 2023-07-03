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
        uint32 numberOfLeaves,
        uint256 index,
        bytes32 leaf
    ) public view returns (bool isMember) {
        if (index >= numberOfLeaves) {
            return false;
        }

        uint256 position = index;
        uint256 width = numberOfLeaves;

        bytes32 computedHash = leaf;

        for (uint256 i = 0; i < proof.length; i++) {
            bytes32 proofElement = proof[i];

            if (position % 2 == 1 || position + 1 == width) {
                computedHash = keccak256(abi.encodePacked(proofElement, computedHash));
            } else {
                computedHash = keccak256(abi.encodePacked(computedHash, proofElement));
            }

            position /= 2;
            width = (width - 1) / 2 + 1;
        }

        return computedHash == getDataRoot(blockNumber);
    }
}
