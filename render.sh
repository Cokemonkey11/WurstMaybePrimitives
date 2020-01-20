#!/usr/bin/env bash

set -euxo pipefail

pushd _gen
cargo run
popd
mv _gen/out.wurst wurst/WurstMaybePrimitives.wurst
