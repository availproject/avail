pragma solidity 0.8.15;

contract DataAvailabilityRouter {
    mapping(uint32 => bytes32) public roots;
}

contract ValidiumContract {

    DataAvailabilityRouter router;

    function setRouter(
        address _router
    ) public {
        router = DataAvailabilityRouter(_router);
    }

    function getDataRoot(
        uint32 blockNumber
    ) public view returns (bytes32) {
        return router.roots(blockNumber);
    }

    function checkDataRootMembership(
        uint32 blockNumber,
        bytes32[] memory proof,
        uint256 numberOfLeaves,
        uint256 leafIndex,
        bytes32 leaf
    ) public view returns (bool) {
        if (leafIndex >= numberOfLeaves) {
            return false;
        }

        uint256 position = leafIndex;
        uint256 width = numberOfLeaves;

        bytes32 computedHash = leaf;

        for (uint256 i = 0; i < proof.length; i++) {
            bytes32 proofElement = proof[i];

            if (position % 2 == 1 || position + 1 == width) {
                computedHash = sha256(abi.encodePacked(proofElement, computedHash));
            } else {
                computedHash = sha256(abi.encodePacked(computedHash, proofElement));
            }

            position /= 2;
            width = (width - 1) / 2 + 1;
        }

        return computedHash == getDataRoot(blockNumber);
    }
}
