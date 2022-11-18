#!/bin/bash
DIR=$(pwd)
P='$PATH'
CONTENT="export PATH="$P:$DIR""
echo $DIR
echo $CONTENT

if [ -z "`grep "$CONTENT" /etc/profile`" ]; then
    echo $CONTENT >> /etc/profile
    source /etc/profile
    echo "$CONTENT -> /etc/profile"
else 
    echo 'file /etc/profile already has variables'
fi

if [ -z "`grep "$CONTENT" /root/.bashrc`" ]; then
    echo $CONTENT >> /root/.bashrc
    source /root/.bashrc
    echo "$CONTENT -> /root/.bashrc"
else 
    echo 'file /root/.bashrc already has variables'
    exit 0

fi