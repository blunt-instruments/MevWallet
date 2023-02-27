#!/bin/sh

rm -rf out/bindings
rm -rf src/bindings
/Users/james/devel/foundry/target/debug/forge bind --module --via-ir
cp -r ./out/bindings ./src/