// static/js/wallet_bridge.js
// Small wallet bridge for MetaMask (window.ethereum).
// Exports functions used by wasm via wasm-bindgen module import.
// Also attaches a window.walletBridge object for convenience.

async function _ensureEthereum() {
  if (!window.ethereum) {
    throw new Error("No injected Ethereum provider found (MetaMask).");
  }
  return window.ethereum;
}

export async function requestAccounts() {
  const eth = await _ensureEthereum();
  const accounts = await eth.request({ method: "eth_requestAccounts" });
  return accounts;
}

// returns accounts array
export async function getAccounts() {
  const eth = await _ensureEthereum();
  return await eth.request({ method: "eth_accounts" });
}

// personal_sign: params [message, account]
export async function signMessage(message) {
  const eth = await _ensureEthereum();
  const accounts = await eth.request({ method: "eth_requestAccounts" });
  const account = accounts[0];
  // personal_sign expects (message, account)
  // many wallets require message to be hex or utf8; personal_sign will accept utf8.
  const sig = await eth.request({
    method: "personal_sign",
    params: [message, account]
  });
  return { signature: sig, account };
}

// EIP-712 (eth_signTypedData_v4) - messageJson should be a stringified JSON
export async function signTypedData(account, messageJson) {
  const eth = await _ensureEthereum();
  const sig = await eth.request({
    method: "eth_signTypedData_v4",
    params: [account, messageJson]
  });
  return { signature: sig, account };
}

// helper: detect current chain id
export async function getChainId() {
  const eth = await _ensureEthereum();
  const cid = await eth.request({ method: "eth_chainId" });
  return cid;
}

// attach to window for direct usage if needed
window.walletBridge = {
  requestAccounts,
  getAccounts,
  signMessage,
  signTypedData,
  getChainId
};
