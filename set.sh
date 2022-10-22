#!/bin/bash
SHELL_FOLDER=$(pwd)
export PATH="$PATH:${SHELL_FOLDER}:$PATH">> /etc/profile
source /etc/profile
