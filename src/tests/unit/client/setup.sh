#!/bin/bash

tmpDir=$1
gitRepo=$2
# tmpDir=/tmp/api-tools-test/api-server/
# gitRepo=https://github.com/a-givertzman/api-server.git
#
mkdir -p $tmpDir
git clone -b Socket-message $gitRepo "$tmpDir"
cdir=$PWD
cd $tmpDir
cargo run --release
cd $cdir