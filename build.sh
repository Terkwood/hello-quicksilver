#!/bin/bash

NAME=hello-quicksilver

cargo web build --target=wasm32-unknown-unknown --release &&
  cp target/wasm32-unknown-unknown/release/*.js static/. &&
  cp target/wasm32-unknown-unknown/release/*.wasm static/. &&
  wasm-opt -Oz -o static/$NAME.wasm static/$NAME.wasm &&
  gzip --force -9 static/$NAME.wasm &&
  mv static/$NAME.wasm.gz static/$NAME.wasm
