// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title ProofCortexVerifier
/// @notice On-chain anchor for off-chain AI inference proofs
/// @dev Designed for BlockDAG’s EVM layer – lightweight & auditable.
contract ProofCortexVerifier {

    struct ProofRecord {
        string modelId;
        string version;          // Model version or commit
        bytes32 inputHash;       // Hash of input to the model
        bytes32 outputHash;      // Hash of model output
        bytes32 traceHash;       // Trace/Merkle root of execution steps
        bytes32 proofHash;       // Combined proof bundle hash
        address submitter;       // Off-chain verifier service
        bool valid;              // Off-chain validation result
        uint256 timestamp;       // Anchor time
        bool locked;             // Prevent further updates
    }

    mapping(bytes32 => ProofRecord) public proofs;

    // Optional: enterprise setups can restrict who may post proofs
    address public trustedSubmitter;

    event ProofSubmitted(
        bytes32 indexed proofId,
        address indexed submitter,
        string modelId,
        bool valid
    );

    event ProofUpdated(bytes32 indexed proofId, bool valid, uint256 timestamp);
    event ProofAnchored(bytes32 indexed proofId, bytes32 proofHash, uint256 time);

    constructor(address _trustedSubmitter) {
        trustedSubmitter = _trustedSubmitter;
    }

    modifier onlyTrusted() {
        require(msg.sender == trustedSubmitter, "Not authorized");
        _;
    }

    function submitProof(
        string calldata modelId,
        string calldata version,
        bytes32 inputHash,
        bytes32 outputHash,
        bytes32 traceHash,
        bytes32 proofHash,
        bool valid
    ) external onlyTrusted returns (bytes32 proofId) {

        proofId = keccak256(
            abi.encodePacked(modelId, version, proofHash, msg.sender)
        );

        require(proofs[proofId].submitter == address(0), "Proof exists");

        proofs[proofId] = ProofRecord({
            modelId: modelId,
            version: version,
            inputHash: inputHash,
            outputHash: outputHash,
            traceHash: traceHash,
            proofHash: proofHash,
            submitter: msg.sender,
            valid: valid,
            timestamp: block.timestamp,
            locked: false
        });

        emit ProofSubmitted(proofId, msg.sender, modelId, valid);
        emit ProofAnchored(proofId, proofHash, block.timestamp);
    }

    function updateProofResult(bytes32 proofId, bool newValidity)
        external
        onlyTrusted
    {
        ProofRecord storage p = proofs[proofId];
        require(p.submitter != address(0), "Not found");
        require(!p.locked, "Immutable");

        p.valid = newValidity;
        p.timestamp = block.timestamp;

        emit ProofUpdated(proofId, newValidity, block.timestamp);
    }

    function lockProof(bytes32 proofId) external onlyTrusted {
        proofs[proofId].locked = true;
    }

    function getProof(bytes32 proofId)
        external
        view
        returns (ProofRecord memory)
    {
        return proofs[proofId];
    }

    function isProofValid(bytes32 proofId) external view returns (bool) {
        return proofs[proofId].valid;
    }
}
