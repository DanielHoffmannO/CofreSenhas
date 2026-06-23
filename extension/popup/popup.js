const API = "http://localhost:5050/api";

const $ = (id) => document.getElementById(id);

async function init() {
  const { token } = await browser.storage.local.get("token");
  if (token) {
    showSenhasView(token);
  } else {
    $("login-view").classList.remove("hidden");
  }
}

$("btn-login").addEventListener("click", async () => {
  const email = $("email").value;
  const senha = $("senha").value;
  $("error").style.display = "none";

  try {
    const res = await fetch(`${API}/auth/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email, senha }),
    });

    if (!res.ok) {
      $("error").textContent = "Credenciais inválidas";
      $("error").style.display = "block";
      return;
    }

    const data = await res.json();
    if (data.twoFactorRequired) {
      $("error").textContent = "2FA não suportado na extensão ainda";
      $("error").style.display = "block";
      return;
    }

    await browser.storage.local.set({ token: data.token });
    showSenhasView(data.token);
  } catch {
    $("error").textContent = "Erro ao conectar na API";
    $("error").style.display = "block";
  }
});

$("btn-logout").addEventListener("click", async () => {
  await browser.storage.local.remove("token");
  $("senhas-view").classList.add("hidden");
  $("login-view").classList.remove("hidden");
});

async function showSenhasView(token) {
  $("login-view").classList.add("hidden");
  $("senhas-view").classList.remove("hidden");

  const tabs = await browser.tabs.query({ active: true, currentWindow: true });
  const url = new URL(tabs[0].url);
  const domain = url.hostname;
  $("domain-info").textContent = `Senhas para: ${domain}`;

  try {
    const res = await fetch(`${API}/senhas`, {
      headers: { Authorization: `Bearer ${token}` },
    });

    if (res.status === 401) {
      await browser.storage.local.remove("token");
      $("senhas-view").classList.add("hidden");
      $("login-view").classList.remove("hidden");
      return;
    }

    const senhas = await res.json();
    const filtradas = senhas.filter(
      (s) => s.url && s.url.includes(domain)
    );
    const outras = senhas.filter(
      (s) => !s.url || !s.url.includes(domain)
    );

    const list = $("senhas-list");
    list.innerHTML = "";

    if (filtradas.length === 0 && outras.length === 0) {
      list.innerHTML = '<div class="empty">Nenhuma senha salva</div>';
      return;
    }

    if (filtradas.length > 0) {
      filtradas.forEach((s) => renderSenha(list, s));
    }

    if (outras.length > 0) {
      if (filtradas.length > 0) {
        const sep = document.createElement("div");
        sep.style.cssText = "text-align:center;color:#999;font-size:11px;margin:8px 0;";
        sep.textContent = "— outras senhas —";
        list.appendChild(sep);
      }
      outras.forEach((s) => renderSenha(list, s));
    }

    list.querySelectorAll(".btn-fill").forEach((btn) => {
      btn.addEventListener("click", () => {
        const login = btn.dataset.login;
        const senha = btn.dataset.senha;
        browser.tabs.sendMessage(tabs[0].id, { action: "fill", login, senha });
        window.close();
      });
    });
  } catch {
    $("senhas-list").innerHTML = '<div class="empty">Erro ao buscar senhas</div>';
  }
}

function renderSenha(list, s) {
  const div = document.createElement("div");
  div.className = "senha-item";
  div.innerHTML = `
    <div class="titulo">${esc(s.titulo)}</div>
    <div class="login">${esc(s.login)}</div>
    <button class="btn-fill" data-login="${esc(s.login)}" data-senha="${esc(s.senha)}">Preencher</button>
  `;
  list.appendChild(div);
}

function esc(str) {
  if (!str) return "";
  const el = document.createElement("span");
  el.textContent = str;
  return el.innerHTML;
}

init();
