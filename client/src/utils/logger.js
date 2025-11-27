class Logger {
  constructor(logElementId) {
    this.logElement = document.getElementById(logElementId);
  }

  log(message, type = 'info') {
    const timestamp = new Date().toLocaleTimeString();
    const logEntry = document.createElement('div');
    logEntry.className = `log-entry ${type}`;
    logEntry.innerHTML = `
      <span class="log-timestamp">[${timestamp}]</span>
      <span>${message}</span>
    `;

    if (this.logElement) {
      this.logElement.appendChild(logEntry);
      this.logElement.scrollTop = this.logElement.scrollHeight;
    }
  }

  info(message) {
    this.log(message, 'info');
  }

  success(message) {
    this.log(message, 'success');
  }

  error(message) {
    this.log(message, 'error');
  }

  warning(message) {
    this.log(message, 'warning');
  }

  clear() {
    if (this.logElement) {
      this.logElement.innerHTML = '';
    }
  }
}

export default Logger;
