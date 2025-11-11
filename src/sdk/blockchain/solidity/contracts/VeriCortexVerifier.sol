// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract VeriCortexVerifier {
    struct Proof {
        string modelId;
        bytes32 inputHash;
        bytes32 outputHash;
        address submitter;
        bool verified;
    }

    mapping(bytes32 => Proof) public proofs;
    event ProofSubmitted(bytes32 indexed proofId, address indexed submitter, string modelId);
    event ProofVerified(bytes32 indexed proofId, bool success);

    function submitProof(
        string memory modelId,
        bytes32 inputHash,
        bytes32 outputHash
    ) public returns (bytes32 proofId) {
        proofId = keccak256(abi.encodePacked(modelId, inputHash, outputHash, msg.sender));
        require(proofs[proofId].submitter == address(0), "Proof already submitted");
        proofs[proofId] = Proof(modelId, inputHash, outputHash, msg.sender, false);
        emit ProofSubmitted(proofId, msg.sender, modelId);
    }

    function verifyProof(bytes32 proofId, bytes32 expectedOutputHash)
        public
        returns (bool)
    {
        Proof storage p = proofs[proofId];
        require(p.submitter != address(0), "Proof not found");
        bool success = (p.outputHash == expectedOutputHash);
        p.verified = success;
        emit ProofVerified(proofId, success);
        return success;
    }
}
