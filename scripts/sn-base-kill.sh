#!/bin/bash

BUILD_DIR=$1

if [ -z $BUILD_DIR ]; then
    BUILD_DIR="$PWD/out"
fi

PID=$(cat $BUILD_DIR/anvil_pid 2> /dev/null)

if [ ! -z $PID ]; then
    SYS_PID=$(ps -fe | grep anvil | awk '{print $2}' | head -n 1)
    if [ "$SYS_PID" = "$PID" ]; then
        # sanity kill
        kill -9 $PID
    fi
    rm $BUILD_DIR/anvil_pid
    echo "killed anvil pid $PID..."
else
    echo "no anvil running..."
fi
