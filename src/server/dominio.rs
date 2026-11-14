/// Extrai o host (minúsculo, sem `www.`, sem porta) de uma URL salva, que
/// pode vir sem esquema (`github.com/login`).
fn host_da_url(url: &str) -> Option<String> {
    let url = url.trim();
    let sem_esquema = url.split_once("://").map_or(url, |(_, resto)| resto);
    let autoridade = sem_esquema.split(['/', '?', '#']).next()?;
    let sem_usuario = autoridade.rsplit('@').next()?;
    let host = sem_usuario.split(':').next()?.to_lowercase();
    let host = host.strip_prefix("www.").unwrap_or(&host).to_string();
    (!host.is_empty()).then_some(host)
}

/// A URL salva casa com a página se o host for idêntico ou se a página for
/// subdomínio dele. `banco.com.evil.net` NÃO casa com `banco.com`.
/// Espelha `urlCombinaComDominio` de `extension/config.js`.
pub fn url_combina_com_host(url_salva: &str, host_pagina: &str) -> bool {
    let (Some(salvo), Some(pagina)) = (host_da_url(url_salva), host_da_url(host_pagina)) else {
        return false;
    };
    pagina == salvo || pagina.ends_with(&format!(".{salvo}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casa_host_igual_www_e_subdominio() {
        assert!(url_combina_com_host(
            "https://github.com/login",
            "github.com"
        ));
        assert!(url_combina_com_host("github.com", "www.github.com"));
        assert!(url_combina_com_host(
            "https://github.com",
            "gist.github.com"
        ));
        assert!(url_combina_com_host(
            "https://Banco.com:8443/x",
            "banco.com"
        ));
    }

    #[test]
    fn rejeita_dominios_maliciosos() {
        assert!(!url_combina_com_host(
            "https://banco.com",
            "banco.com.evil.net"
        ));
        assert!(!url_combina_com_host("https://banco.com", "evilbanco.com"));
        assert!(!url_combina_com_host(
            "https://banco.com",
            "banco.com@evil.net"
        ));
        assert!(!url_combina_com_host(
            "https://banco.com@evil.net/",
            "banco.com"
        ));
        assert!(!url_combina_com_host("", "banco.com"));
        assert!(!url_combina_com_host("https://banco.com", ""));
    }
}
