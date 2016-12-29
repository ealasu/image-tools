#!/usr/bin/env bash

export C_INCLUDE_PATH="$C_INCLUDE_PATH:/usr/local/opt/jpeg-turbo/include/"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"

bindgen src/bindings.h --output src/bindings.rs --convert-macros
