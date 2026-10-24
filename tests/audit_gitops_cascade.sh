#!/usr/bin/env bash
# ==============================================================================
# tests/audit_gitops_cascade.sh
# Valida integracao de workflow GitOps e cascata em joaju-slides (Ticket #3)
# ==============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

WORKFLOW=".forgejo/workflows/ci.yaml"

if [ -f ".forgejo/workflows/publish.yaml" ]; then
  echo "✘ [FALHA] publish.yaml legado para crates.io ainda presente!"
  exit 1
fi

if [ ! -f "$WORKFLOW" ]; then
  echo "✘ [FALHA] $WORKFLOW inexistente!"
  exit 1
fi

python3 -c "
try:
    import yaml
    yaml.safe_load(open('$WORKFLOW'))
except ImportError:
    pass
" || {
  echo "✘ [FALHA] $WORKFLOW possui sintaxe YAML invalida!"
  exit 1
}

if ! grep -q "ubuntu-latest" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao utiliza runner ubuntu-latest!"
  exit 1
fi

if ! grep -q "workflow_dispatch:" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao declara 'workflow_dispatch:'!"
  exit 1
fi

if ! grep -q -E "sparse\+https://(gambi|git)\.2lp\.in/api/packages/lumen/cargo/" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao configura endpoint sparse do registry lumen!"
  exit 1
fi

if ! grep -q "cargo publish --registry lumen" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao publica no registry soberano lumen!"
  exit 1
fi

if ! grep -q "rustup component add" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao adiciona componentes rustfmt/clippy!"
  exit 1
fi

if ! grep -q "CANONICAL_VERSION" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao sincroniza a versao canonica!"
  exit 1
fi

if ! grep -q "repos/lumenpink/joaju/actions/workflows/ci.yaml/dispatches" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao dispara a esteira downstream do joaju!"
  exit 1
fi

echo "✔ [PASS] Auditoria GitOps cascade em joaju-slides validada com sucesso."
