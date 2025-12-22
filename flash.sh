#!/bin/bash

# Little cargo runner to flash and reboot via picotool

mv "$1" "$1".elf # Add the .elf file extension otherwise picotool freaks out
picotool load "$1".elf
picotool reboot

