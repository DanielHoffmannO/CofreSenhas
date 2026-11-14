use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const TENTATIVAS_LIVRES: u32 = 5;
const BLOQUEIO_BASE: Duration = Duration::from_secs(30);
const BLOQUEIO_MAX: Duration = Duration::from_secs(15 * 60);

struct Entrada {
    falhas: u32,
    bloqueado_ate: Instant,
}

/// Limitador de força bruta em memória: depois de 5 falhas seguidas numa
/// chave (e-mail ou IP), bloqueia com atraso dobrando a cada nova falha
/// (30s, 1min, 2min... até 15min). Reiniciar o servidor zera o estado, o que
/// é aceitável para uso doméstico.
#[derive(Default)]
pub struct Limiter(Mutex<HashMap<String, Entrada>>);

impl Limiter {
    /// Segundos restantes de bloqueio, se a chave estiver bloqueada.
    pub fn bloqueado(&self, chave: &str) -> Option<u64> {
        let mapa = self.0.lock().unwrap();
        let e = mapa.get(chave)?;
        let agora = Instant::now();
        (e.bloqueado_ate > agora).then(|| (e.bloqueado_ate - agora).as_secs() + 1)
    }

    pub fn falhou(&self, chave: &str) {
        let mut mapa = self.0.lock().unwrap();
        let agora = Instant::now();
        if mapa.len() > 10_000 {
            mapa.retain(|_, e| e.bloqueado_ate > agora);
        }
        let e = mapa.entry(chave.to_string()).or_insert(Entrada {
            falhas: 0,
            bloqueado_ate: agora,
        });
        e.falhas += 1;
        if e.falhas >= TENTATIVAS_LIVRES {
            let dobras = (e.falhas - TENTATIVAS_LIVRES).min(10);
            let espera = (BLOQUEIO_BASE * 2u32.pow(dobras)).min(BLOQUEIO_MAX);
            e.bloqueado_ate = agora + espera;
        }
    }

    pub fn sucesso(&self, chave: &str) {
        self.0.lock().unwrap().remove(chave);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloqueia_na_quinta_falha_e_libera_no_sucesso() {
        let l = Limiter::default();
        for _ in 0..4 {
            l.falhou("a");
            assert!(l.bloqueado("a").is_none());
        }
        l.falhou("a");
        assert!(l.bloqueado("a").is_some());
        assert!(l.bloqueado("b").is_none());
        l.sucesso("a");
        assert!(l.bloqueado("a").is_none());
    }
}
