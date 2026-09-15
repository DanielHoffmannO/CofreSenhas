use axum::Json;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::server::auth::CurrentUser;
use crate::server::error::{AppError, Result};
use crate::server::models::{ForcaSenha, GerarSenhaRequest, GerarSenhaResponse};

/// Lista de palavras comuns em português, sem acentos (pra digitar em
/// qualquer teclado/dispositivo sem susto), embutida no binário em tempo de
/// compilação -- nenhum arquivo externo, nenhuma tabela no banco.
const PALAVRAS_PT: &str = include_str!("../wordlist_pt.txt");

// `_user` não é lido, mas o extractor `CurrentUser` continua exigindo um
// JWT válido -- é o que mantém esta rota autenticada.
pub async fn gerar(
    _user: CurrentUser,
    Json(req): Json<GerarSenhaRequest>,
) -> Result<Json<GerarSenhaResponse>> {
    let senha = match req {
        GerarSenhaRequest::Aleatorio {
            tamanho,
            usar_maiusculas,
            usar_numeros,
            usar_especiais,
        } => gerar_aleatoria(tamanho, usar_maiusculas, usar_numeros, usar_especiais),
        GerarSenhaRequest::Palavras {
            quantidade,
            capitalizar,
            incluir_numero,
        } => gerar_por_palavras(quantidade, capitalizar, incluir_numero)?,
    };

    let forca = calcular_forca(&senha);
    Ok(Json(GerarSenhaResponse { senha, forca }))
}

fn gerar_aleatoria(
    tamanho: usize,
    usar_maiusculas: bool,
    usar_numeros: bool,
    usar_especiais: bool,
) -> String {
    let mut chars: Vec<char> = ('a'..='z').collect();
    if usar_maiusculas {
        chars.extend('A'..='Z');
    }
    if usar_numeros {
        chars.extend('0'..='9');
    }
    if usar_especiais {
        chars.extend("!@#$%^&*()_+-=[]{}|;:,.<>?".chars());
    }

    let mut rng = rand::thread_rng();
    (0..tamanho)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Estilo "diceware": sorteia N palavras (com repetição -- é isso que torna
/// a matemática de entropia simples: log2(nº de palavras) bits por palavra
/// sorteada) e junta com hífen, tipo `Cavalo-Jardim-Fogueira-42`.
fn gerar_por_palavras(
    quantidade: usize,
    capitalizar: bool,
    incluir_numero: bool,
) -> Result<String> {
    let quantidade = quantidade.clamp(3, 8);
    let palavras: Vec<&str> = PALAVRAS_PT.lines().collect();
    if palavras.is_empty() {
        return Err(AppError::Internal("lista de palavras vazia".into()));
    }

    let mut rng = rand::thread_rng();
    let escolhidas: Vec<String> = (0..quantidade)
        .map(|_| {
            let palavra = palavras.choose(&mut rng).copied().unwrap_or("senha");
            if capitalizar {
                capitalizar_palavra(palavra)
            } else {
                palavra.to_string()
            }
        })
        .collect();

    let mut senha = escolhidas.join("-");
    if incluir_numero {
        senha.push_str(&format!("-{}", rng.gen_range(10..100)));
    }
    Ok(senha)
}

fn capitalizar_palavra(palavra: &str) -> String {
    let mut chars = palavra.chars();
    match chars.next() {
        Some(primeira) => primeira.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Mesma heurística de pontuação do `GeradorSenhaService.CalcularForca`
/// original: um ponto por critério atendido, mapeado para uma categoria.
/// Serve tanto pra senhas aleatórias quanto pra frases de palavras -- uma
/// frase longa com maiúsculas, número e hífen (não-alfanumérico) já bate os
/// mesmos critérios "honestamente".
fn calcular_forca(senha: &str) -> ForcaSenha {
    let mut score = 0;
    if senha.len() >= 8 {
        score += 1;
    }
    if senha.len() >= 12 {
        score += 1;
    }
    if senha.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }
    if senha.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }
    if senha.chars().any(|c| !c.is_alphanumeric()) {
        score += 1;
    }

    match score {
        0..=1 => ForcaSenha::Fraca,
        2 => ForcaSenha::Media,
        3 => ForcaSenha::Forte,
        _ => ForcaSenha::MuitoForte,
    }
}
