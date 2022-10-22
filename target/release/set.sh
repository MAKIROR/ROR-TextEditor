#!/bin/bash
DIR=$(pwd)
P='$PATH'
echo $DIR
CONTENT="export PATH="$DIR:$P""
echo $CONTENT
echo $CONTENT >> /etc/profile
source /etc/profile