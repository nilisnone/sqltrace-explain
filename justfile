set dotenv-load := false
set shell := ["bash", "-uc"]
set positional-arguments := true
default_origin := 'origin'

# 默认命令
test-all:
    cargo test --color=always --package soi --test config_test "" --no-fail-fast -- -Z unstable-options --show-output --test-threads=1
