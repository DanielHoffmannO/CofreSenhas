// Content script — escuta mensagem do popup e preenche campos de login
browser.runtime.onMessage.addListener((msg) => {
  if (msg.action !== "fill") return;

  const passField = document.querySelector('input[type="password"]');
  if (!passField) return;

  // Encontra o campo de login: input anterior ao password que seja text/email
  const inputs = [...document.querySelectorAll("input")];
  const passIdx = inputs.indexOf(passField);
  let loginField = null;

  for (let i = passIdx - 1; i >= 0; i--) {
    const t = inputs[i].type;
    if (t === "text" || t === "email") {
      loginField = inputs[i];
      break;
    }
  }

  if (loginField) {
    setNativeValue(loginField, msg.login);
  }
  setNativeValue(passField, msg.senha);
});

// Seta valor de forma que frameworks (React, Vue) reconheçam a mudança
function setNativeValue(el, value) {
  const setter = Object.getOwnPropertyDescriptor(
    Object.getPrototypeOf(el), "value"
  )?.set;

  if (setter) {
    setter.call(el, value);
  } else {
    el.value = value;
  }

  el.dispatchEvent(new Event("input", { bubbles: true }));
  el.dispatchEvent(new Event("change", { bubbles: true }));
}
