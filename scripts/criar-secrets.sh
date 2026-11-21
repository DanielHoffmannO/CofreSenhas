#!/usr/bin/env bash
# Cria secrets/jwt_secret e secrets/encryption_secret a partir do .env atual,
# SEM imprimir os valores, e confere que o arquivo é idêntico ao valor do .env
# (um byte a mais no ENCRYPTION_SECRET deixaria o banco ilegível).
# Depois, para o container (uid 10001) conseguir ler e mais ninguém:
#   sudo chown 10001:10001 secrets/* && sudo chmod 400 secrets/*
set -euo pipefail
cd "$(dirname "$0")/.."
umask 077
mkdir -p secrets
set -a; . ./.env; set +a
for par in "jwt_secret:JWT_SECRET" "encryption_secret:ENCRYPTION_SECRET"; do
  arq="secrets/${par%%:*}"; var="${par##*:}"
  [[ -n "${!var:-}" ]] || { echo "$var vazio no .env" >&2; exit 1; }
  printf '%s' "${!var}" > "$arq"
  [[ "$(cat "$arq")" == "${!var}" ]] || { echo "$arq difere do .env" >&2; exit 1; }
  echo "ok: $arq ($(stat -c %s "$arq") bytes)"
done
