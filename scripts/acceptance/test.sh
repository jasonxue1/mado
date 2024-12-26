#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)
PROJECT_ROOT=($SCRIPT_DIR/../../)
DATA_ROOT=$SCRIPT_DIR/data
DOC_PATH=$DATA_ROOT/markdownlint/test/rule_tests
TEMP_PATH=$PROJECT_ROOT/tmp

mdl --config $SCRIPT_DIR/.mdlrc $DOC_PATH > $TEMP_PATH/mdl.txt
cargo run check --output-format=mdl $DOC_PATH > $TEMP_PATH/mado.txt

# Truncate unnecessary texts
sed -i '' -e "/^Further documentation is available for these failures:/d" $TEMP_PATH/mdl.txt > /dev/null
sed -i '' -e "/^ - /d" $TEMP_PATH/mdl.txt > /dev/null
sed -i '' -e "/^Found /d" $TEMP_PATH/mado.txt > /dev/null
