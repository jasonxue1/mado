#!/bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)
DATA_ROOT=$SCRIPT_DIR/data

cd $DATA_ROOT
git clone --sparse --filter=blob:none https://github.com/markdownlint/markdownlint.git
cd markdownlint
git sparse-checkout set test/rule_tests
git checkout

# TODO: Support each styles
style_files=$(find data/markdownlint/test/rule_tests -name '*_style.rb')
for style_file in $style_files; do
  if [ $style_file -eq "data/markdownlint/test/rule_tests/default_test_style.rb" ]; then
    continue
  fi

  markdown_file=$(echo $style_file | sed 's/_style.rb$/.md/')
  mv $markdown_file $markdown_file.bak
done
