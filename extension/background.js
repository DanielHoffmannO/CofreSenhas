// Background script — mantém token ativo e limpa se expirar
browser.storage.onChanged.addListener((changes) => {
  if (changes.token && !changes.token.newValue) {
    console.log("CofreSenhas: token removido");
  }
});
