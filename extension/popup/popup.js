const $ = (id) => document.getElementById(id);
const enviar = (msg) => browser.runtime.sendMessage(msg);

function erro(texto) {
  $("error").textContent = texto;
  $("error").style.display = "block";
}

async function init() {
  const { logado } = await enviar({ action: "status" });
  if (logado) mostrarSenhas();
  else $("login-view").classList.remove("hidden");
}

$("btn-login").addEventListener("click", async () => {
  $("error").style.display = "none";
  const r = await enviar({
    action: "login",
    email: $("email").value,
    senha: $("senha").value,
    totpCode: $("totp").value.trim(),
  });
  if (r?.twoFactorRequired) {
    $("totp").classList.remove("hidden");
    $("totp").focus();
    return erro("Informe o código 2FA (ou um código de recuperação)");
  }
  if (!r?.ok) return erro(r?.erro || "Erro ao conectar na API");
  $("senha").value = "";
  $("totp").value = "";
  mostrarSenhas();
});

$("btn-logout").addEventListener("click", async () => {
  await enviar({ action: "logout" });
  $("senhas-view").classList.add("hidden");
  $("login-view").classList.remove("hidden");
});

async function mostrarSenhas() {
  $("login-view").classList.add("hidden");
  $("senhas-view").classList.remove("hidden");

  const [tab] = await browser.tabs.query({ active: true, currentWindow: true });
  let host = "";
  try {
    const u = new URL(tab.url);
    host = u.protocol === "https:" ? u.hostname : "";
  } catch {}
  $("domain-info").textContent = host ? `Senhas para: ${host}` : "Abra uma página https para preencher";

  const list = $("senhas-list");
  list.innerHTML = "";
  const itens = host ? await enviar({ action: "matches", tabId: tab.id }) : [];
  if (!itens || itens.length === 0) {
    list.innerHTML = '<div class="empty">Nenhuma senha salva para este site</div>';
    return;
  }
  itens.forEach((s) => {
    const div = document.createElement("div");
    div.className = "senha-item";
    const titulo = document.createElement("div");
    titulo.className = "titulo";
    titulo.textContent = s.titulo;
    const login = document.createElement("div");
    login.className = "login";
    login.textContent = s.login;
    const btn = document.createElement("button");
    btn.className = "btn-fill";
    btn.textContent = "Preencher";
    btn.addEventListener("click", async () => {
      await enviar({ action: "preencher", id: s.id, tabId: tab.id });
      window.close();
    });
    div.append(titulo, login, btn);
    list.appendChild(div);
  });
}

init();
