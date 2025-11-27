class APIService {
  constructor(baseURL, logger) {
    this.baseURL = baseURL;
    this.logger = logger;
  }

  async request(endpoint, options = {}) {
    const url = `${this.baseURL}${endpoint}`;

    try {
      this.logger.info(`Calling ${options.method || 'GET'} ${endpoint}`);

      const response = await fetch(url, {
        ...options,
        headers: {
          'Content-Type': 'application/json',
          ...options.headers,
        },
      });

      const data = await response.json();

      if (!response.ok) {
        this.logger.warning(`Response ${response.status}: ${data.message || data.error}`);
      } else {
        this.logger.success(`Response ${response.status}: OK`);
      }

      return {
        status: response.status,
        ok: response.ok,
        data,
      };
    } catch (error) {
      this.logger.error(`Request failed: ${error.message}`);
      throw error;
    }
  }

  async health() {
    return this.request('/api/health');
  }

  async free() {
    return this.request('/api/free');
  }

  async paid(paymentHeader = null) {
    const options = {};

    if (paymentHeader) {
      options.headers = {
        'x-payment': paymentHeader,
      };
    }

    return this.request('/api/paid', options);
  }
}

export default APIService;
