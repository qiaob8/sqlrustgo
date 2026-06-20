#!/bin/bash
# scripts/run_gates.sh
# SQLRustGo 发布门禁脚本（v5 规范要求）
# 配套清单：docs/releases/v2.6.0/RELEASE_GATE_CHECKLIST.md
# 配套报告：reports/week-14/week-14-实验报告.md

set -e

echo "=== Running Gate Checks ==="
echo "Date : $(date)"
echo "Branch: $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo N/A)"
echo "Commit: $(git rev-parse HEAD 2>/dev/null || echo N/A)"
echo ""

# ---------- Code Gates ----------
echo "[1/6] Building..."
cargo build --release

echo "[2/6] Running tests..."
cargo test --all-features

echo "[3/6] Running Clippy..."
cargo clippy --all-features -- -D warnings

echo "[4/6] Checking format..."
cargo fmt --check --all

# ---------- Quality Gates ----------
# 注：cargo-tarpaulin 在 Linux/macOS 稳定，Windows 需要 WSL 或 grcov 替代
echo "[5/6] Checking coverage..."
if command -v cargo-tarpaulin >/dev/null 2>&1; then
    cargo tarpaulin --out Xml --packages parser,executor,storage
else
    echo "[warn] cargo-tarpaulin 未安装，跳过覆盖率检查（Linux/WSL 工具）"
fi

# ---------- Security Gates ----------
echo "[6/6] Running security audit..."
cargo audit

echo "=== All Gates Passed ==="
