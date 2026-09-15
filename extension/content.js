// Content script — detecta campos de senha na página e mostra um ícone de
// sugestão (Shadow DOM, isolado do CSS do site) quando existem credenciais
// salvas para o domínio atual. Também continua atendendo ao preenchimento
// manual disparado pelo botão "Preencher" do popup.
(function () {
  const processedFields = new WeakSet();
  let credenciaisPromise = null;

  function getCredenciais() {
    if (!credenciaisPromise) {
      credenciaisPromise = browser.storage.local.get("token").then(({ token }) => {
        if (!token) return [];
        return fetch(`${API}/senhas`, { headers: { Authorization: `Bearer ${token}` } })
          .then((res) => (res.ok ? res.json() : []))
          .catch(() => []);
      });
    }
    return credenciaisPromise;
  }

  function encontrarCampoLogin(passField) {
    const inputs = [...document.querySelectorAll("input")];
    const passIdx = inputs.indexOf(passField);
    for (let i = passIdx - 1; i >= 0; i--) {
      const t = inputs[i].type;
      if (t === "text" || t === "email") return inputs[i];
    }
    return null;
  }

  function setNativeValue(el, value) {
    const setter = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(el), "value")?.set;
    if (setter) setter.call(el, value);
    else el.value = value;
    el.dispatchEvent(new Event("input", { bubbles: true }));
    el.dispatchEvent(new Event("change", { bubbles: true }));
  }

  function preencher(passField, loginField, cred) {
    if (loginField) setNativeValue(loginField, cred.login);
    setNativeValue(passField, cred.senha);
    fecharDropdown();
  }

  function escapeHtml(str) {
    const el = document.createElement("span");
    el.textContent = str ?? "";
    return el.innerHTML;
  }

  let dropdownAtual = null;
  function fecharDropdown() {
    if (dropdownAtual) {
      dropdownAtual.remove();
      dropdownAtual = null;
    }
  }

  function abrirDropdown(passField, loginField, matches) {
    fecharDropdown();
    const host = document.createElement("div");
    host.style.cssText = "position:fixed;z-index:2147483647;";
    const shadow = host.attachShadow({ mode: "open" });

    const itensHtml = matches
      .map(
        (m, i) => `
        <div class="item" data-idx="${i}">
          <div class="titulo">${escapeHtml(m.titulo)}</div>
          <div class="login">${escapeHtml(m.login)}</div>
        </div>`
      )
      .join("");

    shadow.innerHTML = `
      <style>
        .painel { min-width:200px; max-width:280px; background:#fff; border-radius:8px;
          box-shadow:0 4px 16px rgba(0,0,0,.25); font-family:-apple-system,sans-serif; overflow:hidden; }
        .item { padding:8px 10px; cursor:pointer; border-bottom:1px solid #eee; }
        .item:last-child { border-bottom:none; }
        .item:hover { background:#f0f0f5; }
        .titulo { font-size:13px; font-weight:600; color:#1a1a2e; }
        .login { font-size:11px; color:#666; }
      </style>
      <div class="painel">${itensHtml}</div>
    `;

    document.body.appendChild(host);
    const r = passField.getBoundingClientRect();
    host.style.top = `${r.bottom + 4}px`;
    host.style.left = `${r.left}px`;

    shadow.querySelectorAll(".item").forEach((el) => {
      el.addEventListener("click", () => preencher(passField, loginField, matches[Number(el.dataset.idx)]));
    });

    dropdownAtual = host;
    setTimeout(() => document.addEventListener("click", fecharDropdown, { once: true }), 0);
  }

  function posicionarSobreCampo(el, campo) {
    const atualizar = () => {
      const r = campo.getBoundingClientRect();
      el.style.top = `${r.top + (r.height - 22) / 2}px`;
      el.style.left = `${r.right - 26}px`;
    };
    atualizar();
    window.addEventListener("scroll", atualizar, true);
    window.addEventListener("resize", atualizar);
  }

  function criarIcone(passField, loginField, matches) {
    const host = document.createElement("div");
    host.style.cssText = "position:fixed;width:22px;height:22px;z-index:2147483647;cursor:pointer;";
    const shadow = host.attachShadow({ mode: "open" });
    shadow.innerHTML = `
      <style>
        .icone { width:22px; height:22px; border-radius:4px; background:#1a1a2e;
          display:flex; align-items:center; justify-content:center;
          box-shadow:0 1px 3px rgba(0,0,0,.3); font-size:12px; }
      </style>
      <div class="icone" title="CofreSenhas: preencher (${matches.length})">🔐</div>
    `;
    document.body.appendChild(host);
    posicionarSobreCampo(host, passField);

    host.addEventListener("click", (e) => {
      e.stopPropagation();
      dropdownAtual ? fecharDropdown() : abrirDropdown(passField, loginField, matches);
    });
  }

  function processarCampo(passField) {
    if (processedFields.has(passField)) return;
    processedFields.add(passField);

    getCredenciais().then((lista) => {
      if (!lista.length) return;
      const dominio = location.hostname;
      const matches = lista.filter((s) => s.url && s.url.includes(dominio));
      if (!matches.length) return;
      criarIcone(passField, encontrarCampoLogin(passField), matches);
    });
  }

  function escanear() {
    document.querySelectorAll('input[type="password"]').forEach(processarCampo);
  }

  escanear();
  // Muitos logins (SPAs, React/Vue) renderizam o formulário depois do load.
  new MutationObserver(escanear).observe(document.documentElement, { childList: true, subtree: true });

  // Compatibilidade com o preenchimento manual via botão "Preencher" do popup.
  browser.runtime.onMessage.addListener((msg) => {
    if (msg.action !== "fill") return;
    const passField = document.querySelector('input[type="password"]');
    if (!passField) return;
    preencher(passField, encontrarCampoLogin(passField), { login: msg.login, senha: msg.senha });
  });
})();
