const Senhas = {
    async render() {
        Layout.render(`
            <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:20px">
                <h2>🗝️ Minhas Senhas</h2>
                <button class="btn" onclick="Senhas.showForm()">+ Nova</button>
            </div>
            <div id="senha-form" class="card hidden">
                <div class="form-group"><label>Título</label><input id="s-titulo" placeholder="Gmail"></div>
                <div class="form-group"><label>Login</label><input id="s-login" placeholder="user@email.com"></div>
                <div class="form-group"><label>Senha</label><input id="s-senha" placeholder="••••••"></div>
                <div class="form-group"><label>URL</label><input id="s-url" placeholder="https://..."></div>
                <div class="form-group"><label>Notas</label><input id="s-notas" placeholder="Opcional"></div>
                <button class="btn" onclick="Senhas.criar()">Salvar</button>
                <button class="btn btn-outline" onclick="Senhas.hideForm()">Cancelar</button>
            </div>
            <div id="senhas-list" class="grid"></div>
        `);
        await this.listar();
    },

    showForm() { document.getElementById('senha-form').classList.remove('hidden'); },
    hideForm() { document.getElementById('senha-form').classList.add('hidden'); },

    async listar() {
        const senhas = await Api.get('/api/senhas');
        const container = document.getElementById('senhas-list');
        if (!senhas || senhas.length === 0) {
            container.innerHTML = '<p class="text-center" style="color:var(--text-muted)">Nenhuma senha salva.</p>';
            return;
        }
        container.innerHTML = senhas.map(s => `
            <div class="card senha-item">
                <div class="info">
                    <h3>${s.titulo}</h3>
                    <p>👤 ${s.login}</p>
                    <p>🔑 <span style="color:var(--success)">${s.senha}</span></p>
                    ${s.url ? `<p>🔗 ${s.url}</p>` : ''}
                </div>
                <div class="actions">
                    <button class="btn btn-sm btn-danger" onclick="Senhas.deletar(${s.id})">🗑️</button>
                </div>
            </div>
        `).join('');
    },

    async criar() {
        const body = {
            titulo: document.getElementById('s-titulo').value,
            login: document.getElementById('s-login').value,
            senha: document.getElementById('s-senha').value,
            url: document.getElementById('s-url').value || null,
            notas: document.getElementById('s-notas').value || null
        };
        await Api.post('/api/senhas', body);
        this.hideForm();
        await this.listar();
    },

    async deletar(id) {
        if (!confirm('Deletar esta senha?')) return;
        await Api.delete(`/api/senhas/${id}`);
        await this.listar();
    }
};
