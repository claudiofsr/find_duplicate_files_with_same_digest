#!/usr/bin/env bash

# find-identical-files --generate=bash > completions/completion_derive.bash
# find-identical-files --generate=elvish > completions/completion_derive.elvish
# find-identical-files --generate=fish > completions/completion_derive.fish
# find-identical-files --generate=powershell > completions/completion_derive.powershell
# find-identical-files --generate=zsh > completions/completion_derive.zsh

shells="bash elvish fish powershell zsh"

for shell in $shells; do
 find-identical-files --generate=$shell > completions/completion_derive.$shell
done
