#!/usr/bin/env bash
# Backup diário do CofreSenhas: snapshot consistente do SQLite, cifrado,
# com retenção e cópia opcional para FORA do servidor.
#
# Variáveis (de /etc/cofresenhas-backup.env ou do ambiente):
#   BACKUP_DIR            destino local (padrão: ./backups)
#   AGE_RECIPIENT         chave pública age (preferido); ou
#   BACKUP_PASSPHRASE_FILE arquivo (chmod 600) com a senha do gpg simétrico
#   RETENCAO_DIAS         quantos dias manter (padrão: 30)
#   BACKUP_REMOTE         destino rsync/scp fora do servidor (ex.: user@maquina:/pasta/)
#   COMPOSE_DIR           pasta do docker-compose.yml (padrão: dir deste script/..)
#
# O backup NÃO inclui o ENCRYPTION_SECRET (sem ele o banco é ilegível):
# guarde-o à parte, fora do servidor, e nunca junto do arquivo cifrado.
set -euo pipefail

COMPOSE_DIR="${COMPOSE_DIR:-$(cd "$(dirname "$0")/.." && pwd)}"
BACKUP_DIR="${BACKUP_DIR:-$COMPOSE_DIR/backups}"
RETENCAO_DIAS="${RETENCAO_DIAS:-30}"
DESTINO="$BACKUP_DIR/cofre-$(date +%Y%m%d-%H%M%S).db"

umask 077
mkdir -p "$BACKUP_DIR"
cd "$COMPOSE_DIR"

if [[ -n "${AGE_RECIPIENT:-}" ]]; then
  CIFRAR=(age -r "$AGE_RECIPIENT"); EXT=age
elif [[ -n "${BACKUP_PASSPHRASE_FILE:-}" ]]; then
  CIFRAR=(gpg --batch --yes --pinentry-mode loopback --passphrase-file "$BACKUP_PASSPHRASE_FILE" --symmetric --cipher-algo AES256); EXT=gpg
else
  echo "defina AGE_RECIPIENT ou BACKUP_PASSPHRASE_FILE (backup sem cifra é recusado)" >&2; exit 1
fi

# Snapshot dentro do container, lido por stdout e cifrado sem tocar o disco em claro.
docker compose exec -T api sh -c 'f=$(mktemp -u /tmp/b.XXXXXX) && backup_db /data/cofresenhas.db "$f" && cat "$f"; rc=$?; rm -f "$f"; exit $rc' \
  | "${CIFRAR[@]}" > "$DESTINO.$EXT"

[[ -s "$DESTINO.$EXT" ]] || { echo "backup vazio, abortando" >&2; rm -f "$DESTINO.$EXT"; exit 1; }
echo "backup: $DESTINO.$EXT ($(stat -c %s "$DESTINO.$EXT") bytes)"

find "$BACKUP_DIR" -name 'cofre-*.db.*' -mtime +"$RETENCAO_DIAS" -delete

if [[ -n "${BACKUP_REMOTE:-}" ]]; then
  rsync -a "$DESTINO.$EXT" "$BACKUP_REMOTE"
  echo "copiado para $BACKUP_REMOTE"
fi
