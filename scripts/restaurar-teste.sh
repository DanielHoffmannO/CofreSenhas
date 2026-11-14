#!/usr/bin/env bash
# Testa um backup de ponta a ponta SEM tocar no banco real: decifra para um
# diretório temporário, confere a integridade e conta as linhas.
# Uso: restaurar-teste.sh <arquivo.db.age|arquivo.db.gpg> [identidade-age | arquivo-de-senha-gpg]
# Para restaurar de verdade: pare a API, copie o .db decifrado para o volume
# api-data como cofresenhas.db (dono uid 10001) e suba a API de novo.
set -euo pipefail
ARQ="${1:?informe o arquivo de backup}"
CHAVE="${2:-}"
TMP="$(mktemp -d)"; trap 'rm -rf "$TMP"' EXIT
umask 077

case "$ARQ" in
  *.age) age -d -i "${CHAVE:?informe o arquivo de identidade age}" "$ARQ" > "$TMP/restaurado.db" ;;
  *.gpg) gpg --batch --pinentry-mode loopback --passphrase-file "${CHAVE:?informe o arquivo de senha}" -d "$ARQ" > "$TMP/restaurado.db" 2>/dev/null ;;
  *) echo "extensão desconhecida" >&2; exit 1 ;;
esac

python3 -I - "$TMP/restaurado.db" <<'PY'
import sqlite3, sys
c = sqlite3.connect(f"file:{sys.argv[1]}?mode=ro", uri=True)
ok = c.execute("PRAGMA integrity_check").fetchone()[0]
usuarios = c.execute("SELECT COUNT(*) FROM usuarios").fetchone()[0]
senhas = c.execute("SELECT COUNT(*) FROM senhas").fetchone()[0]
print(f"integridade: {ok} | usuarios: {usuarios} | senhas: {senhas}")
sys.exit(0 if ok == "ok" else 1)
PY
echo "restauração de teste OK"
