// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title VeriCortexVerifier
/// @notice Stores results of off-chain verification proofs submitted via CLI
/// @dev Designed for integration with BlockDAG's EVM-compatible layer.
contract VeriCortexVerifier {
    struct ProofRecord {
        string modelId;          // Identifier for the model being verified
        bytes32 proofHash;       // Hash of full proof data (input/output, etc.)
        address submitter;       // Off-chain verifier/agent submitting this proof
        bool valid;              // Verification result
        uint256 timestamp;       // On-chain timestamp when submitted
    }

    mapping(bytes32 => ProofRecord) public proofs;  // proofId => record
    event ProofSubmitted(bytes32 indexed proofId, address indexed submitter, string modelId, bool valid);
    event ProofUpdated(bytes32 indexed proofId, bool valid, uint256 timestamp);

    /// @notice Submit a verified proof result to chain (off-chain verifier already validated it)
    /// @param proofHash Hash of the verified proof
    /// @param modelId Model identifier (matches off-chain verifier input)
    /// @param valid Whether the proof passed verification off-chain
    /// @return proofId A unique on-chain proof identifier
    function submitProof(
        bytes32 proofHash,
        string calldata modelId,
        bool valid
    ) external returns (bytes32 proofId) {
        proofId = keccak256(abi.encodePacked(proofHash, modelId, msg.sender));
        require(proofs[proofId].submitter == address(0), "Proof already exists");

        proofs[proofId] = ProofRecord({
            modelId: modelId,
            proofHash: proofHash,
            submitter: msg.sender,
            valid: valid,
            timestamp: block.timestamp
        });

        emit ProofSubmitted(proofId, msg.sender, modelId, valid);
    }

    /// @notice Allows proof owner or trusted oracle to update result (optional extension)
    function updateProofResult(bytes32 proofId, bool newValidity) external {
        ProofRecord storage record = proofs[proofId];
        require(record.submitter != address(0), "Proof not found");
        require(msg.sender == record.submitter, "Only submitter can update");

        record.valid = newValidity;
        record.timestamp = block.timestamp;

        emit ProofUpdated(proofId, newValidity, record.timestamp);
    }

    /// @notice Get details of any proof
    function getProof(bytes32 proofId) external view returns (ProofRecord memory) {
        return proofs[proofId];
    }

    /// @notice Returns whether a proof has been verified valid
    function isProofValid(bytes32 proofId) external view returns (bool) {
        return proofs[proofId].valid;
    }
}
