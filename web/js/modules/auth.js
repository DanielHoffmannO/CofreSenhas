const Auth = {
    renderLogin() {
        Layout.render(`
            <div class="auth-container">
                <h2>🔐 Login</h2>
                <div id="alert"></div>
                <div class="card">
                    <div class="form-group">
                        <label>Email</label>
                        <input type="email" id="email" placeholder="admin@cofre.com">
                    </div>
                    <div class="form-group">
                        <label>Senha</label>
                        <input type="password" id="senha" placeholder="••••••">
                    </div>
                    <button class="btn" style="width:100%" onclick="Auth.login()">Entrar</button>
                    <p class="text-center mt-20">
                        <a href="#register" style="color:var(--accent)">Criar conta</a>
                    </p>
                </div>
            </div>
        `);
    },

    renderRegister() {
        Layout.render(`
            <div class="auth-container">
                <h2>🔐 Cadastro</h2>
                <div id="alert"></div>
                <div class="card">
                    <div class="form-group">
                        <label>Nome</label>
                        <input type="text" id="nome" placeholder="Seu nome">
                    </div>
                    <div class="form-group">
                        <label>Email</label>
                        <input type="email" id="email" placeholder="email@exemplo.com">
                    </div>
                    <div class="form-group">
                        <label>Senha</label>
                        <input type="password" id="senha" placeholder="••••••">
                    </div>
                    <button class="btn" style="width:100%" onclick="Auth.register()">Cadastrar</button>
                    <p class="text-center mt-20">
                        <a href="#login" style="color:var(--accent)">Já tenho conta</a>
                    </p>
                </div>
            </div>
        `);
    },

    async login() {
        try {
            const email = document.getElementById('email').value;
            const senha = document.getElementById('senha').value;
            const res = await Api.post('/api/auth/login', { email, senha });
            Api.setToken(res.token);
            Router.navigate('senhas');
        } catch (e) {
            document.getElementById('alert').innerHTML = `<div class="alert alert-error">${e.message}</div>`;
        }
    },

    async register() {
        try {
            const nome = document.getElementById('nome').value;
            const email = document.getElementById('email').value;
            const senha = document.getElementById('senha').value;
            const res = await Api.post('/api/auth/register', { nome, email, senha });
            Api.setToken(res.token);
            Router.navigate('senhas');
        } catch (e) {
            document.getElementById('alert').innerHTML = `<div class="alert alert-error">${e.message}</div>`;
        }
    }
};
