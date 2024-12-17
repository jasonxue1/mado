#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)
DATA_ROOT=$SCRIPT_DIR/data

if [ -z ./target/release/downlint ]; then
  cargo build --release
fi

hyperfine --ignore-failure --warmup 5 \
  "./target/release/downlint check $DATA_ROOT/gitlab" \
  "mdl --config $SCRIPT_DIR/.mdlrc $DATA_ROOT/gitlab" \
  "$SCRIPT_DIR/node_modules/.bin/markdownlint --config $SCRIPT_DIR/.markdownlint.jsonc $DATA_ROOT/gitlab"
