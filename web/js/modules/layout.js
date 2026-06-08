const Layout = {
    render(content) {
        const app = document.getElementById('app');
        if (!Api.isAuthenticated()) {
            app.innerHTML = content;
            return;
        }
        app.innerHTML = `
            <header>
                <h1>🔐 Cofre de Senhas</h1>
                <nav>
                    <a href="#senhas">Senhas</a>
                    <a href="#gerador">Gerador</a>
                    <a href="#" onclick="Layout.logout()">Sair</a>
                </nav>
            </header>
            <main>${content}</main>
        `;
    },

    logout() {
        Api.clearToken();
        Router.navigate('login');
    }
};
