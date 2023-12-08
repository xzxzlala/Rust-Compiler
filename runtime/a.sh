#!/bin/bash

nasm -f elf64 compiled_code.s -o compiled_code.o
ar r libcompiled_code.a compiled_code.o 
rustc stub.rs -L . -o stub.exe 
./stub.exe