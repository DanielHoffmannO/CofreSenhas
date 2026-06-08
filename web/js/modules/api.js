const Api = {
    getToken() { return localStorage.getItem('token'); },
    setToken(token) { localStorage.setItem('token', token); },
    clearToken() { localStorage.removeItem('token'); },
    isAuthenticated() { return !!this.getToken(); },

    async request(path, options = {}) {
        const headers = { 'Content-Type': 'application/json' };
        const token = this.getToken();
        if (token) headers['Authorization'] = `Bearer ${token}`;

        const response = await fetch(`${CONFIG.API_URL}${path}`, { ...options, headers });

        if (response.status === 401) {
            this.clearToken();
            Router.navigate('login');
            return null;
        }

        if (!response.ok) {
            const err = await response.text();
            throw new Error(err || `Erro ${response.status}`);
        }

        const text = await response.text();
        return text ? JSON.parse(text) : null;
    },

    get(path) { return this.request(path); },
    post(path, body) { return this.request(path, { method: 'POST', body: JSON.stringify(body) }); },
    put(path, body) { return this.request(path, { method: 'PUT', body: JSON.stringify(body) }); },
    delete(path) { return this.request(path, { method: 'DELETE' }); }
};
