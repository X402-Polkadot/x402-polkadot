import Logger from './utils/logger.js';
import WalletService from './services/wallet.js';
import APIService from './services/api.js';

const SERVER_URL = import.meta.env.VITE_SERVER_URL || 'http://127.0.0.1:3000';
const POLKADOT_NETWORK = import.meta.env.VITE_POLKADOT_NETWORK || 'paseo';
const MNEMONIC = import.meta.env.VITE_SIGNER_MNEMONIC || '';

const logger = new Logger('log-container');
const walletService = new WalletService(logger);
const apiService = new APIService(SERVER_URL, logger);

let currentPaymentRequirements = null;

function showPageLoader(msg = 'Loading...') {
  const overlay = document.createElement('div');
  overlay.id = 'page-loader';
  overlay.style.cssText = 'position:fixed;top:0;left:0;width:100%;height:100%;background:rgba(0,0,0,0.7);z-index:10002;display:flex;align-items:center;justify-content:center;';

  overlay.innerHTML = `
    <div style="background:#fff;padding:40px;border-radius:12px;text-align:center;">
      <div style="width:50px;height:50px;border:4px solid #e0e0e0;border-top-color:#2A5244;border-radius:50%;animation:spin 1s linear infinite;margin:0 auto 20px;"></div>
      <div style="color:#2A5244;font-weight:600;font-size:18px;margin-bottom:8px;">${msg}</div>
      <div style="color:#666;font-size:14px;">Please wait...</div>
    </div>
  `;

  if (!document.querySelector('#loader-spin')) {
    const style = document.createElement('style');
    style.id = 'loader-spin';
    style.textContent = '@keyframes spin { to { transform: rotate(360deg); }}';
    document.head.appendChild(style);
  }

  document.body.appendChild(overlay);
}

function hidePageLoader() {
  const el = document.getElementById('page-loader');
  if (el) {
    el.style.opacity = '0';
    el.style.transition = 'opacity 0.3s';
    setTimeout(() => el.remove(), 300);
  }
}

function showNotification(msg, type = 'error') {
  const notif = document.createElement('div');
  const color = type === 'error' ? '#c62828' : '#2A5244';
  notif.style.cssText = `position:fixed;top:50%;left:50%;transform:translate(-50%,-50%);background:#fff;padding:25px 30px;border-radius:10px;box-shadow:0 10px 40px rgba(0,0,0,0.2);z-index:10001;max-width:400px;border-left:4px solid ${color};`;

  notif.innerHTML = `
    <div style="display:flex;align-items:center;gap:12px;">
      <div style="font-size:24px;">${type === 'error' ? '⚠️' : 'ℹ️'}</div>
      <div>
        <div style="color:${color};font-weight:600;font-size:16px;margin-bottom:4px;">${type === 'error' ? 'Error' : 'Notice'}</div>
        <div style="color:#1a1a1a;font-size:14px;">${msg}</div>
      </div>
    </div>
  `;

  document.body.appendChild(notif);
  setTimeout(() => {
    notif.style.opacity = '0';
    notif.style.transition = 'opacity 0.2s';
    setTimeout(() => notif.remove(), 200);
  }, 3000);
}

function updateUIFromEnv() {
  // Show mnemonic
  const mnemonicEl = document.getElementById('wallet-mnemonic');
  if (mnemonicEl && MNEMONIC) {
    mnemonicEl.textContent = MNEMONIC;
  }

  // Show server URL in the config section
  const apiUrlEl = document.getElementById('api-url');
  if (apiUrlEl) {
    apiUrlEl.value = SERVER_URL;
  }
}

function showReceiverInfo(recipient) {
  const receiverSection = document.getElementById('receiver-section');
  const receiverAddress = document.getElementById('receiver-address');
  const receiverExplorerLink = document.getElementById('receiver-explorer-link');

  if (receiverSection && receiverAddress) {
    receiverAddress.textContent = recipient;
    receiverSection.classList.remove('hidden');
  }

  if (receiverExplorerLink) {
    receiverExplorerLink.href = `https://paseo.subscan.io/account/${recipient}`;
    receiverExplorerLink.style.display = 'inline';
  }
}

function showResult(elementId, data, type = 'info') {
  const element = document.getElementById(elementId);
  if (!element) {
    return;
  }

  element.classList.add('show');
  element.innerHTML = `<pre>${JSON.stringify(data, null, 2)}</pre>`;
}

async function connectWallet() {
  try {
    await walletService.connect();
    logger.success('Wallet connected');
  } catch (error) {
    logger.error(`Connection failed: ${error.message}`);
    showNotification(error.message, 'error');
  }
}

async function refresh() {
  const btn = document.getElementById('refresh-btn');
  btn.disabled = true;
  btn.textContent = 'Refreshing...';

  try {
    // Reload env values to UI
    updateUIFromEnv();

    // Reconnect wallet and fetch balance
    if (!walletService.isConnected()) {
      await connectWallet();
    } else {
      await walletService.fetchBalance();
    }

    logger.success('Refreshed');
  } catch (error) {
    logger.error(`Refresh failed: ${error.message}`);
  } finally {
    btn.disabled = false;
    btn.textContent = 'Refresh';
  }
}

async function testHealth() {
  try {
    const response = await apiService.health();
    showResult('health-result', response.data, response.ok ? 'success' : 'error');
  } catch (error) {
    showResult('health-result', { error: error.message }, 'error');
  }
}

async function testFree() {
  try {
    const response = await apiService.free();
    showResult('free-result', response.data, response.ok ? 'success' : 'error');
  } catch (error) {
    showResult('free-result', { error: error.message }, 'error');
  }
}

async function testPaid() {
  try {
    const response = await apiService.paid();

    if (response.status === 402) {
      logger.warning('Payment required');
      currentPaymentRequirements = response.data.paymentRequirements;
      showResult('paid-result', response.data, 'info');

      // Show receiver info dynamically from payment requirements
      showReceiverInfo(currentPaymentRequirements.recipient);

      await showAutoPaymentWarning(currentPaymentRequirements);
    } else {
      showResult('paid-result', response.data, response.ok ? 'success' : 'error');
    }
  } catch (error) {
    showResult('paid-result', { error: error.message }, 'error');
  }
}

async function showAutoPaymentWarning(requirements) {
  if (!walletService.isConnected()) {
    logger.error('Wallet not connected');
    showNotification('Wallet not connected!', 'error');
    return;
  }

  const amount = (requirements.amount / 10_000_000_000).toFixed(2);
  const popup = document.createElement('div');
  popup.id = 'payment-popup';
  popup.style.cssText = 'position:fixed;top:50%;left:50%;transform:translate(-50%,-50%);background:#fff;padding:35px;border-radius:12px;box-shadow:0 10px 40px rgba(0,0,0,0.2);z-index:10000;max-width:650px;border:2px solid #2A5244;';

  popup.innerHTML = `
    <div>
      <h2 style="color:#2A5244;margin:0 0 10px 0;font-size:22px;">Payment Required</h2>
      <div style="background:#f5f5f5;padding:15px;border-radius:8px;margin-bottom:20px;">
        <div style="display:flex;justify-content:space-between;margin-bottom:10px;">
          <span style="color:#666;font-weight:500;">Amount:</span>
          <span style="color:#1a1a1a;font-weight:600;">${amount} PAS</span>
        </div>
        <div>
          <div style="color:#666;font-weight:500;margin-bottom:4px;">To:</div>
          <div style="color:#1a1a1a;font-weight:600;font-size:13px;font-family:monospace;word-break:break-all;background:#fff;padding:8px;border-radius:4px;border:1px solid #e0e0e0;">${requirements.recipient}</div>
        </div>
      </div>
      <div id="payment-status" style="background:#fafafa;padding:15px;border-radius:8px;margin-bottom:15px;min-height:100px;max-height:200px;overflow-y:auto;"></div>
      <div style="background:#fff3cd;border-left:4px solid #ffc107;padding:12px;border-radius:4px;font-size:13px;color:#856404;">
        <strong>Demo:</strong> In production this happens automatically in the background.
      </div>
    </div>
  `;

  document.body.appendChild(popup);
  await autoProcessPayment(requirements, popup);
}

function updatePaymentStatus(status, done = false, err = false) {
  const container = document.getElementById('payment-status');
  if (!container) return;

  const item = document.createElement('div');
  const color = err ? '#c62828' : done ? '#2e7d32' : '#1a1a1a';
  const bg = err ? '#ffebee' : done ? '#e8f5e9' : '#f5f5f5';
  const icon = err ? 'X' : done ? '✓' : '›';

  item.style.cssText = `color:${color};font-weight:${done||err?'600':'500'};margin:8px 0;font-size:14px;padding:8px 12px;background:${bg};border-radius:6px;border-left:3px solid ${color};`;
  item.innerHTML = `<span style="margin-right:8px;font-weight:bold;">${icon}</span>${status}`;

  container.appendChild(item);
  container.scrollTop = container.scrollHeight;
}

async function autoProcessPayment(requirements, popup) {
  try {
    updatePaymentStatus('Signing transaction...');
    logger.info('Signing transaction');

    const signedTxHex = await walletService.signTransaction({
      to: requirements.recipient,
      amount: requirements.amount,
    });

    updatePaymentStatus('Transaction signed');
    updatePaymentStatus('Submitting to server...');
    logger.info('Submitting payment');

    const response = await apiService.paid(signedTxHex);

    if (response.ok) {
      updatePaymentStatus('Verifying on blockchain...');
      updatePaymentStatus('Waiting for confirmation...');

      await new Promise(resolve => setTimeout(resolve, 500));
      updatePaymentStatus('Transaction confirmed', true);

      logger.success('Payment successful');
      logger.info(`Tx: ${response.data.transaction_hash}`);
      showResult('paid-result', response.data, 'success');

      setTimeout(async () => {
        popup.style.borderColor = '#4caf50';
        popup.querySelector('h2').textContent = 'Payment Successful!';
        popup.querySelector('h2').style.color = '#4caf50';
      }, 500);

      setTimeout(async () => {
        popup.remove();
        await walletService.fetchBalance();
        logger.success('Balance updated');
      }, 3000);
    } else {
      updatePaymentStatus('Payment failed', false, true);
      logger.error('Payment failed');
      showResult('paid-result', response.data, 'error');

      setTimeout(() => popup.remove(), 3000);
    }
  } catch (error) {
    updatePaymentStatus(`Error: ${error.message}`, false, true);
    logger.error(`Payment error: ${error.message}`);
    showResult('paid-result', { error: error.message }, 'error');

    setTimeout(() => popup.remove(), 3000);
  }
}

async function payAndRetry() {
  const btn = document.getElementById('pay-btn');
  const originalText = btn.textContent;

  if (!currentPaymentRequirements) {
    logger.error('No payment requirements');
    return;
  }

  if (!walletService.isConnected()) {
    logger.error('Wallet not connected');
    showNotification('Wallet not connected!', 'error');
    return;
  }

  try {
    btn.disabled = true;
    btn.textContent = 'Signing...';
    logger.info('Signing transaction');

    const signedTxHex = await walletService.signTransaction({
      to: currentPaymentRequirements.recipient,
      amount: currentPaymentRequirements.amount,
    });

    btn.textContent = 'Submitting...';
    logger.info('Submitting payment');

    const response = await apiService.paid(signedTxHex);

    if (response.ok) {
      logger.success('Payment successful');
      logger.info(`Tx: ${response.data.transaction_hash}`);
      showResult('paid-result', response.data, 'success');
      document.getElementById('payment-requirements').classList.add('hidden');
      currentPaymentRequirements = null;
      btn.textContent = 'Payment Complete';
    } else {
      logger.error('Payment failed');
      showResult('paid-result', response.data, 'error');
      btn.disabled = false;
      btn.textContent = originalText;
    }
  } catch (error) {
    logger.error(`Payment error: ${error.message}`);
    showResult('paid-result', { error: error.message }, 'error');
    btn.disabled = false;
    btn.textContent = originalText;
  }
}

function clearLogs() {
  logger.clear();
  logger.info('Ready');
}

// Event listeners
document.getElementById('refresh-btn').addEventListener('click', refresh);
document.getElementById('test-health-btn').addEventListener('click', testHealth);
document.getElementById('test-free-btn').addEventListener('click', testFree);
document.getElementById('test-paid-btn').addEventListener('click', testPaid);
document.getElementById('pay-btn').addEventListener('click', payAndRetry);
document.getElementById('clear-logs-btn').addEventListener('click', clearLogs);

// Initialize on page load
logger.info('Ready');

// Show env values in UI
updateUIFromEnv();

// Auto-connect wallet on page load
showPageLoader('Connecting...');
setTimeout(async () => {
  try {
    await connectWallet();
  } finally {
    hidePageLoader();
  }
}, 500);
