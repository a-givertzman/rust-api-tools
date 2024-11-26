#!/bin/bash
#
tmpDir=$1
# tmpDir=/tmp/api-tools-test/api-server/
# gitRepo=https://github.com/a-givertzman/api-server.git
#
exit 0
cdir=$PWD
cd $tmpDir
echo "Executing ApiServer in '$tmpDir'..."
cargo run --release
cd $cdir