// Content script: detecta campos de login e mostra um ícone de sugestão
// (Shadow DOM, isolado do CSS do site) quando existem credenciais salvas para
// o domínio. Cobre login em duas etapas (e-mail numa tela, senha na seguinte,
// sem recarregar) e SPAs: um laço leve reavalia a página, porque campos
// nascem, somem e se movem durante animações. A senha só chega aqui depois do
// clique do usuário e é pedida ao background, que revalida o host.
(function () {
  const ehHttps = location.protocol === "https:";
  const ICONE = 22;
  const rastreados = new Map(); // campo -> { host, tipo }
  let credenciaisPromise = null;
  let dropdownAtual = null;

  // Só o background fala com a API; aqui chegam só título/login (sem senha).
  function getCredenciais() {
    if (!ehHttps) return Promise.resolve([]);
    if (!credenciaisPromise) {
      credenciaisPromise = browser.runtime.sendMessage({ action: "matches" }).then((l) => l || []);
    }
    return credenciaisPromise;
  }

  function visivel(el) {
    if (!el.isConnected) return false;
    const r = el.getBoundingClientRect();
    if (r.width < 4 || r.height < 4) return false;
    if (el.closest('[aria-hidden="true"], [hidden]')) return false;
    const cs = getComputedStyle(el);
    return cs.visibility !== "hidden" && cs.display !== "none" && cs.opacity !== "0";
  }

  function camposSenha() {
    return [...document.querySelectorAll('input[type="password"]')].filter(visivel);
  }

  const NOME_USUARIO = /user|login|email|e-mail|ident|account|conta/i;
  function ehCampoUsuario(el) {
    if (el.disabled || el.readOnly) return false;
    const t = (el.type || "text").toLowerCase();
    if (t !== "text" && t !== "email" && t !== "tel") return false;
    const ac = (el.getAttribute("autocomplete") || "").toLowerCase();
    if (/\busername\b|\bemail\b/.test(ac)) return true;
    if (t === "email") return true;
    return NOME_USUARIO.test(`${el.name} ${el.id}`);
  }

  // Campo de usuário "ligado" a um campo de senha: o último campo de texto
  // visível antes dele no documento.
  function usuarioDaSenha(passField) {
    const inputs = [...document.querySelectorAll("input")];
    for (let i = inputs.indexOf(passField) - 1; i >= 0; i--) {
      const t = inputs[i].type;
      if ((t === "text" || t === "email" || t === "tel") && visivel(inputs[i])) return inputs[i];
    }
    return null;
  }

  function setNativeValue(el, value) {
    el.focus();
    const setter = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(el), "value")?.set;
    if (setter) setter.call(el, value);
    else el.value = value;
    el.dispatchEvent(new Event("input", { bubbles: true }));
    el.dispatchEvent(new Event("change", { bubbles: true }));
  }

  function fecharDropdown() {
    if (dropdownAtual) {
      dropdownAtual.remove();
      dropdownAtual = null;
    }
  }

  function escapeHtml(str) {
    const el = document.createElement("span");
    el.textContent = str ?? "";
    return el.innerHTML;
  }

  function abrirDropdown(campo, tipo, matches) {
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
      <div class="painel">${itensHtml}</div>`;
    document.body.appendChild(host);
    const r = campo.getBoundingClientRect();
    host.style.top = `${r.bottom + 4}px`;
    host.style.left = `${r.left}px`;

    shadow.querySelectorAll(".item").forEach((el) => {
      el.addEventListener("click", async () => {
        const id = matches[Number(el.dataset.idx)].id;
        // No campo de usuário só o login é pedido: a senha fica no background
        // até existir um campo de senha visível.
        const cred = await browser.runtime.sendMessage({ action: "credencial", id, soLogin: tipo === "usuario" });
        if (cred) preencher(campo, tipo, cred);
      });
    });
    dropdownAtual = host;
    setTimeout(() => document.addEventListener("click", fecharDropdown, { once: true }), 0);
  }

  function preencher(campo, tipo, cred) {
    if (tipo === "usuario") {
      setNativeValue(campo, cred.login);
    } else {
      const usuario = usuarioDaSenha(campo);
      if (usuario && cred.login) setNativeValue(usuario, cred.login);
      setNativeValue(campo, cred.senha);
    }
    fecharDropdown();
  }

  function criarIcone(campo, tipo, matches) {
    const host = document.createElement("div");
    host.style.cssText = `position:fixed;width:${ICONE}px;height:${ICONE}px;z-index:2147483647;cursor:pointer;display:none;`;
    const shadow = host.attachShadow({ mode: "open" });
    shadow.innerHTML = `
      <style>
        .icone { width:${ICONE}px; height:${ICONE}px; border-radius:4px; background:#1a1a2e;
          display:flex; align-items:center; justify-content:center;
          box-shadow:0 1px 3px rgba(0,0,0,.3); font-size:12px; }
      </style>
      <div class="icone" title="CofreSenhas: preencher (${matches.length})">🔐</div>`;
    document.body.appendChild(host);
    host.addEventListener("click", (e) => {
      e.stopPropagation();
      dropdownAtual ? fecharDropdown() : abrirDropdown(campo, tipo, matches);
    });
    return host;
  }

  async function registrar(campo, tipo) {
    if (rastreados.has(campo)) return;
    rastreados.set(campo, { host: null, tipo }); // evita pedir duas vezes
    const matches = await getCredenciais();
    if (!matches.length) return;
    const e = rastreados.get(campo);
    if (e) e.host = criarIcone(campo, tipo, matches);
  }

  // Reavalia a página: registra campos novos e acompanha posição/visibilidade
  // dos ícones (o campo da senha do Google, por exemplo, desliza para dentro).
  function tick() {
    if (!ehHttps) return;
    const senhas = camposSenha();
    senhas.forEach((c) => registrar(c, "senha"));
    // Etapa só de e-mail: sem campo de senha visível, o campo de usuário é a âncora.
    if (!senhas.length) {
      document.querySelectorAll("input").forEach((c) => {
        if (ehCampoUsuario(c) && visivel(c)) registrar(c, "usuario");
      });
    }
    for (const [campo, e] of rastreados) {
      if (!campo.isConnected) {
        e.host?.remove();
        rastreados.delete(campo);
        continue;
      }
      if (!e.host) continue;
      const mostrar = visivel(campo) && (e.tipo === "senha" || !senhas.length);
      e.host.style.display = mostrar ? "block" : "none";
      if (mostrar) {
        const r = campo.getBoundingClientRect();
        e.host.style.top = `${r.top + (r.height - ICONE) / 2}px`;
        e.host.style.left = `${r.right - ICONE - 4}px`;
      }
    }
  }

  if (ehHttps) {
    tick();
    setInterval(tick, 400);
    new MutationObserver(tick).observe(document.documentElement, { childList: true, subtree: true });
  }

  // Preenchimento manual pelo popup: usa o campo visível (nunca o oculto).
  browser.runtime.onMessage.addListener((msg) => {
    if (msg.action !== "fill" || !ehHttps) return;
    const senha = camposSenha()[0];
    if (senha) return preencher(senha, "senha", { login: msg.login, senha: msg.senha });
    const usuario = [...document.querySelectorAll("input")].find((c) => ehCampoUsuario(c) && visivel(c));
    if (usuario) preencher(usuario, "usuario", { login: msg.login });
  });
})();
