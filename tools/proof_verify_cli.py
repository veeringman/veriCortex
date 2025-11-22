#!/usr/bin/env python3
"""
proofcortex-cli.py

Simple CLI to read proofs from the ProofCortex / VeriCortexVerifier contract on BlockDAG.
Usage:
  export .env with BLOCKDAG_RPC and VERIFIER_ADDRESS
  pip install web3 python-dotenv click
  python tools/proofcortex-cli.py read <proofId>
  python tools/proofcortex-cli.py listen  # listen ProofSubmitted events
"""

import os
import json
import click
from web3 import Web3
from dotenv import load_dotenv
from eth_abi import decode_abi

load_dotenv()

RPC = os.getenv("BLOCKDAG_RPC")
CONTRACT = os.getenv("VERIFIER_ADDRESS")

if not RPC or not CONTRACT:
    # allow runtime override but warn
    pass

# Minimal ABI (matches contract functions used)
ABI = [
    {
      "inputs": [
        { "internalType": "bytes32", "name": "proofId", "type": "bytes32" }
      ],
      "name": "getProof",
      "outputs": [
        {
          "components": [
            {"internalType":"string","name":"modelId","type":"string"},
            {"internalType":"bytes32","name":"proofHash","type":"bytes32"},
            {"internalType":"address","name":"submitter","type":"address"},
            {"internalType":"bool","name":"valid","type":"bool"},
            {"internalType":"uint256","name":"timestamp","type":"uint256"}
          ],
          "internalType":"struct VeriCortexVerifier.ProofRecord",
          "name":"",
          "type":"tuple"
        }
      ],
      "stateMutability":"view",
      "type":"function"
    },
    {
      "inputs":[{"internalType":"bytes32","name":"proofId","type":"bytes32"}],
      "name":"isProofValid",
      "outputs":[{"internalType":"bool","name":"","type":"bool"}],
      "stateMutability":"view",
      "type":"function"
    },
    {
      "anonymous": False,
      "inputs": [
        {"indexed": True, "internalType": "bytes32", "name": "proofId", "type": "bytes32"},
        {"indexed": True, "internalType": "address", "name": "submitter", "type": "address"},
        {"indexed": False, "internalType": "string", "name": "modelId", "type": "string"},
        {"indexed": False, "internalType": "bool", "name": "valid", "type": "bool"}
      ],
      "name": "ProofSubmitted",
      "type": "event"
    }
]

def get_web3(rpc_url=None):
    url = rpc_url or RPC
    if not url:
        raise RuntimeError("BLOCKDAG_RPC not set in .env or CLI. Set BLOCKDAG_RPC env var.")
    return Web3(Web3.HTTPProvider(url))

def get_contract(w3, address=None):
    addr = (address or CONTRACT)
    if not addr:
        raise RuntimeError("VERIFIER_ADDRESS not set in .env or CLI.")
    return w3.eth.contract(address=w3.to_checksum_address(addr), abi=ABI)

@click.group()
def cli():
    pass

@cli.command()
@click.argument("proofid")
@click.option("--rpc", default=None, help="RPC URL override")
@click.option("--contract", default=None, help="Verifier contract address override")
def read(proofid, rpc, contract):
    """Read a proof record by proofId (hex bytes32)."""
    w3 = get_web3(rpc)
    c = get_contract(w3, contract)
    # accept proofid as hex string with or without 0x
    pid = proofid if proofid.startswith("0x") else "0x" + proofid
    try:
        proof = c.functions.getProof(pid).call()
    except Exception as e:
        click.echo(f"Error fetching proof: {e}")
        return
    # proof is tuple (modelId, proofHash, submitter, valid, timestamp)
    modelId, proofHash, submitter, valid, timestamp = proof
    click.echo("=== Proof Record ===")
    click.echo(f"proofId     : {pid}")
    click.echo(f"modelId     : {modelId}")
    click.echo(f"proofHash   : {proofHash.hex() if hasattr(proofHash,'hex') else proofHash}")
    click.echo(f"submitter   : {submitter}")
    click.echo(f"valid       : {valid}")
    click.echo(f"timestamp   : {timestamp}")
    status = c.functions.isProofValid(pid).call()
    click.echo(f"onchain valid?: {status}")

@cli.command()
@click.option("--rpc", default=None, help="RPC URL override")
@click.option("--contract", default=None, help="Verifier contract address override")
def listen(rpc, contract):
    """Listen for ProofSubmitted events (long-running)."""
    w3 = get_web3(rpc)
    c = get_contract(w3, contract)
    click.echo("Listening for ProofSubmitted events. CTRL+C to exit.")
    # create filter from latest block
    event_filter = c.events.ProofSubmitted.createFilter(fromBlock='latest')
    try:
        while True:
            for ev in event_filter.get_new_entries():
                args = ev['args']
                click.echo("---- ProofSubmitted ----")
                click.echo(f"proofId: {args.proofId.hex()}")
                click.echo(f"submitter: {args.submitter}")
                click.echo(f"modelId: {args.modelId}")
                click.echo(f"valid: {args.valid}")
    except KeyboardInterrupt:
        click.echo("Stopped listening.")
    except Exception as e:
        click.echo(f"Error while listening: {e}")

if __name__ == "__main__":
    cli()
