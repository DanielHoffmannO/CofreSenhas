const Gerador = {
    render() {
        Layout.render(`
            <h2 style="margin-bottom:20px">⚡ Gerador de Senhas</h2>
            <div class="card">
                <div class="slider-container">
                    <label>Tamanho: <span id="tamanho-val" class="slider-value">16</span></label>
                    <input type="range" id="tamanho" min="4" max="64" value="16" oninput="Gerador.updateSlider()">
                </div>
                <div class="checkbox-group">
                    <label><input type="checkbox" id="maiusculas" checked> Maiúsculas</label>
                    <label><input type="checkbox" id="numeros" checked> Números</label>
                    <label><input type="checkbox" id="especiais" checked> Especiais</label>
                </div>
                <button class="btn" style="width:100%;margin-top:15px" onclick="Gerador.gerar()">Gerar Senha</button>
            </div>
            <div id="gerador-result" class="hidden">
                <div class="gerador-output" id="senha-gerada"></div>
                <div class="text-center">
                    <span id="forca-badge"></span>
                    <button class="btn btn-sm btn-outline" style="margin-left:10px" onclick="Gerador.copiar()">📋 Copiar</button>
                </div>
            </div>
        `);
    },

    updateSlider() {
        document.getElementById('tamanho-val').textContent = document.getElementById('tamanho').value;
    },

    async gerar() {
        const body = {
            tamanho: parseInt(document.getElementById('tamanho').value),
            usarMaiusculas: document.getElementById('maiusculas').checked,
            usarNumeros: document.getElementById('numeros').checked,
            usarEspeciais: document.getElementById('especiais').checked
        };
        const res = await Api.post('/api/gerador', body);
        document.getElementById('senha-gerada').textContent = res.senha;
        const forcas = ['Fraca', 'Média', 'Forte', 'Muito Forte'];
        document.getElementById('forca-badge').innerHTML = `<span class="forca-badge forca-${res.forca}">${forcas[res.forca]}</span>`;
        document.getElementById('gerador-result').classList.remove('hidden');
    },

    copiar() {
        const senha = document.getElementById('senha-gerada').textContent;
        navigator.clipboard.writeText(senha);
    }
};
