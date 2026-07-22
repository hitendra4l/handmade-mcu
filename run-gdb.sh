#!/usr/bin/env bash
# cargo runner: flash the ELF, then launch gdb server + client.
# $1 = binary path cargo appends.
set -e
probe-rs download --chip STM32F446RE "$1"
exec probe-rs gdb --chip STM32F446RE --reset-halt --gdb gdb-multiarch "$1"
