// Config do background script (único que fala com a API).
const API = "https://cofre.hoffserv.lan/api";

// A URL salva casa com a página se o host for idêntico ou se a página for
// subdomínio dele (gist.github.com casa com github.com). Comparar com
// `includes` deixaria "banco.com.evil.net" receber a senha de "banco.com".
function urlCombinaComDominio(urlSalva, hostnameDaPagina) {
  if (!urlSalva) return false;
  const comEsquema = /^[a-z][a-z0-9+.-]*:\/\//i.test(urlSalva) ? urlSalva : `https://${urlSalva}`;
  let hostSalvo;
  try {
    hostSalvo = new URL(comEsquema).hostname.toLowerCase().replace(/^www\./, "");
  } catch {
    return false;
  }
  const hostPagina = hostnameDaPagina.toLowerCase().replace(/^www\./, "");
  return hostPagina === hostSalvo || hostPagina.endsWith(`.${hostSalvo}`);
}
