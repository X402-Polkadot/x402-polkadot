import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { findHealthyNode } from '../config/networks.js';

class WalletService {
  constructor(logger) {
    this.logger = logger;
    this.keypair = null;
    this.api = null;
    this.network = import.meta.env.VITE_POLKADOT_NETWORK || 'paseo';
    this.currentNode = null;
  }

  async connect() {
    try {
      await cryptoWaitReady();

      const mnemonic = import.meta.env.VITE_SIGNER_MNEMONIC;
      if (!mnemonic) {
        throw new Error('Mnemonic not configured');
      }

      const keyring = new Keyring({ type: 'sr25519' });
      this.keypair = keyring.addFromMnemonic(mnemonic);

      // Find healthy node silently
      this.currentNode = await findHealthyNode(this.network);

      const provider = new WsProvider(this.currentNode.url);
      this.api = await ApiPromise.create({ provider });

      await this.fetchBalance();

      return { address: this.keypair.address };
    } catch (error) {
      this.logger.error(`Connection failed: ${error.message}`);
      throw error;
    }
  }

  async fetchBalance() {
    if (!this.keypair || !this.api) return;

    try {
      const { data: balance } = await this.api.query.system.account(this.keypair.address);
      const amount = (balance.free.toBigInt() / BigInt(10**10)).toString();
      const formatted = `${amount} PAS`;

      const balanceEl = document.getElementById('dot-balance');
      if (balanceEl) balanceEl.textContent = formatted;

      const addressEl = document.getElementById('wallet-address');
      if (addressEl) addressEl.textContent = this.keypair.address;
    } catch (error) {
      const el = document.getElementById('dot-balance');
      if (el) el.textContent = 'Error';
    }
  }

  async getBalanceForAddress(address) {
    if (!this.api) throw new Error('API not ready');

    try {
      const { data: balance } = await this.api.query.system.account(address);
      const amount = (balance.free.toBigInt() / BigInt(10**10)).toString();
      return `${amount} PAS`;
    } catch (error) {
      return 'Error';
    }
  }

  async fetchReceiverBalance(address) {
    if (!this.api) return;

    try {
      const balance = await this.getBalanceForAddress(address);

      const el = document.getElementById('receiver-balance');
      if (el) el.textContent = balance;
    } catch (error) {
      const el = document.getElementById('receiver-balance');
      if (el) el.textContent = 'Error';
    }
  }

  getCurrentNode() {
    return this.currentNode;
  }

  async signTransaction(tx) {
    if (!this.keypair || !this.api) throw new Error('Wallet not ready');

    try {
      const nonce = await this.api.rpc.system.accountNextIndex(this.keypair.address);
      const transfer = this.api.tx.balances.transferKeepAlive(tx.to, tx.amount);
      const signed = await transfer.signAsync(this.keypair, { nonce });
      const hex = signed.toHex();

      return hex;
    } catch (error) {
      this.logger.error(`Signing failed: ${error.message}`);
      throw error;
    }
  }

  getAddress() {
    return this.keypair?.address || null;
  }

  isConnected() {
    return this.keypair !== null && this.api !== null;
  }

  formatAddress(address) {
    if (!address) return '';
    return `${address.slice(0, 8)}...${address.slice(-8)}`;
  }
}

export default WalletService;
