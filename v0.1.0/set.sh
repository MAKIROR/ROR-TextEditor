#!/bin/bash
DIR=$(pwd)
P='$PATH'
CONTENT="export PATH="$P:$DIR""
echo $DIR
echo $CONTENT

if [ -z "`grep "$CONTENT" /etc/profile`" ]; then
    echo $CONTENT >> /etc/profile
    source /etc/profile
else 
    echo 'file already has variables'
    exit 0
fi