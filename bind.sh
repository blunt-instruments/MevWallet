#!/bin/sh

rm -rf out/bindings
rm -rf src/bindings
forge bind --module --via-ir
cp -r ./out/bindings ./src/