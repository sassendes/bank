#!/usr/bin/env bash
set -e

ROOT="bank"; mkdir -p "$ROOT"; cd "$ROOT"

DOMAINS=(
  ledger accounts parties transactions payments
  identity onboarding
  cards loans deposits
  interest_fees limits_controls
  aml fraud reporting audit
  statements notifications gateway admin webapp treasury
)

for d in "${DOMAINS[@]}"; do
  mkdir -p "services/$d/src" "services/$d/tests"

  # named layer files — no mod.rs
  echo "// $d — domain types (entities, value objects)" > "services/$d/src/model.rs"
  echo "// $d — persistence (DB access)"                 > "services/$d/src/repository.rs"
  echo "// $d — business logic"                          > "services/$d/src/service.rs"
  echo "// $d — HTTP handlers"                           > "services/$d/src/handler.rs"
  echo "// $d — domain events"                           > "services/$d/src/event.rs"

  cat > "services/$d/src/lib.rs" <<EOF
pub mod model;
pub mod repository;
pub mod service;
pub mod handler;
pub mod event;
EOF

  cat > "services/$d/src/main.rs" <<EOF
use ${d}::*;

fn main() {
    println!("$d service");
}
EOF

  echo "// integration tests for $d" > "services/$d/tests/${d}_test.rs"

  cat > "services/$d/Cargo.toml" <<EOF
[package]
name = "$d"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF
done

mkdir -p schema docs scripts

{
  echo "[workspace]"
  echo "resolver = \"2\""
  echo "members = ["
  for d in "${DOMAINS[@]}"; do echo "  \"services/$d\","; done
  echo "]"
} > Cargo.toml

{
  echo "# Bank — domain map"
  echo ""
  echo "Spine first: parties -> accounts -> ledger -> transactions"
  echo ""
  for d in "${DOMAINS[@]}"; do echo "- [ ] $d"; done
} > docs/MAP.md

echo "domains: ${#DOMAINS[@]}"
echo "files: $(find . -type f | wc -l)  (zero mod.rs)"
