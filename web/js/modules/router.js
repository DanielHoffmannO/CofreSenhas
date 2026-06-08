const Router = {
    routes: {},

    register(hash, handler) { this.routes[hash] = handler; },

    navigate(hash) { window.location.hash = `#${hash}`; },

    getCurrentRoute() { return window.location.hash.slice(1) || 'login'; },

    async resolve() {
        const route = this.getCurrentRoute();
        if (!Api.isAuthenticated() && route !== 'login' && route !== 'register') {
            this.navigate('login');
            return;
        }
        if (Api.isAuthenticated() && (route === 'login' || route === 'register')) {
            this.navigate('senhas');
            return;
        }

        const handler = this.routes[route];
        if (handler) await handler();
    },

    init() {
        window.addEventListener('hashchange', () => this.resolve());
        this.resolve();
    }
};
