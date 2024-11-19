#!/bin/bash
#
tmpDir=$1
gitRepo=$2
# tmpDir=/tmp/api-tools-test/api-server/
# gitRepo=https://github.com/a-givertzman/api-server.git
#
# exit 0
echo "Preparing tmp dir for ApiServer in '$tmpDir'..."
rm -rf /tmp/api-tools-test/
mkdir -p $tmpDir
echo "Cloning ApiServer from git repo '$gitRepo'..."
git clone -b "Socket-message" "$gitRepo" "$tmpDir"
cdir=$PWD
cd $tmpDir
echo "Executing ApiServer in '$tmpDir'..."
cargo build --release
cd $cdir