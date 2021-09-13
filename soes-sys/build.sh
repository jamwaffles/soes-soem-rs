#!/usr/bin/env bash

set -xe

pushd SOES

cmake -DCMAKE_CXX_FLAGS="-Wno-error=address-of-packed-member"

# WORKSPACE=$(pwd)

# cmake -E make_directory $WORKSPACE/build

# cmake -B $WORKSPACE/build -S $WORKSPACE -DCMAKE_BUILD_TYPE=Release

# cmake -DWARNINGS_AS_ERRORS=OFF --build $WORKSPACE/build
