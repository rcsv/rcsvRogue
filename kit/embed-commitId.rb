#!/usr/bin/env ruby

puts STDIN.read.gsub('$Gcid$', '$Gcid: ' + `git rev-parse HEAD`.strip + '$')

# **NOTE**: add trigger condition (described below) into `.git/config`
# file.
#
# [filter "embed-commitId"]
# smudge = ruby kit/embed-commitId.rb
# clean = perl -pe \"s/\\\\\\$Gcid[^\\\\\\$]*\\\\\\$/\\\\\\$Gcid\\\\\\$/\"
#
# このスクリプトのトリガーはローカルリポジトリの直下 ".git/config" にある。
# 上記のように記載しておけば、git checkout をすると $Gcid$ に値が入り、
# git add でステージにあげると、$Gcid$ はまた通常の文字列のみに戻る
# git diff でこの部分が差分として出てくることはない。
