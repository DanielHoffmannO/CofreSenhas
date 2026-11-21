// Background script: único lugar que guarda o token e fala com a API.
// O token vive só na memória (some ao fechar o navegador) e os content
// scripts nunca recebem senhas além da credencial que o usuário escolheu.
let token = null;
const permitidas = new Map(); // id -> { url, login, senha } do último `matches`

async function api(caminho, opcoes = {}) {
  const res = await fetch(`${API}${caminho}`, {
    ...opcoes,
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
  });
  if (res.status === 401 && token) sair();
  return res;
}

function sair() {
  token = null;
  permitidas.clear();
}

function hostSeguro(urlDaAba) {
  try {
    const u = new URL(urlDaAba);
    return u.protocol === "https:" ? u.hostname : null;
  } catch {
    return null;
  }
}

// Credenciais cujo domínio casa com o host da aba. O host vem da própria aba
// (não do que o content script afirma) e o servidor já filtra por ele.
async function credenciaisDaAba(tabId) {
  if (!token) return [];
  const tab = await browser.tabs.get(tabId);
  const host = hostSeguro(tab.url);
  if (!host) return [];
  const res = await api(`/senhas?host=${encodeURIComponent(host)}`);
  if (!res.ok) return [];
  const lista = (await res.json()).filter((s) => urlCombinaComDominio(s.url, host));
  lista.forEach((s) => permitidas.set(s.id, { url: s.url, login: s.login, senha: s.senha }));
  return lista;
}

async function tratar(msg, sender) {
  switch (msg.action) {
    case "status":
      return { logado: !!token };

    case "login": {
      const res = await fetch(`${API}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email: msg.email, senha: msg.senha, totpCode: msg.totpCode || null }),
      });
      if (res.status === 429) return { erro: (await res.json()).message };
      if (!res.ok) return { erro: "Credenciais inválidas" };
      const data = await res.json();
      if (data.twoFactorRequired) return { twoFactorRequired: true };
      token = data.token;
      return { ok: true };
    }

    case "logout":
      sair();
      return { ok: true };

    case "matches": {
      const tabId = sender.tab ? sender.tab.id : msg.tabId;
      const lista = await credenciaisDaAba(tabId);
      return lista.map(({ id, titulo, login }) => ({ id, titulo, login }));
    }

    case "credencial": {
      // Revalida o host no momento do clique: a senha só sai para a aba cujo
      // host ainda casa com a URL salva do item.
      const tabId = sender.tab ? sender.tab.id : msg.tabId;
      const item = permitidas.get(msg.id);
      const tab = await browser.tabs.get(tabId);
      const host = hostSeguro(tab.url);
      if (!item || !host || !urlCombinaComDominio(item.url, host)) return null;
      // Etapa só de e-mail: a senha não sai do background.
      return msg.soLogin ? { login: item.login } : { login: item.login, senha: item.senha };
    }

    case "preencher": {
      // Disparado pelo popup: envia login/senha só à aba ativa, se casar.
      const cred = await tratar({ action: "credencial", id: msg.id, tabId: msg.tabId }, {});
      if (!cred) return { ok: false };
      await browser.tabs.sendMessage(msg.tabId, { action: "fill", ...cred });
      return { ok: true };
    }
  }
}

browser.runtime.onMessage.addListener((msg, sender) => tratar(msg, sender).catch(() => null));
