#!/usr/bin/env bash

if [ ! -d wireguard-go ]; then
    git clone https://github.com/iosmanthus/wireguard-go
fi
cd wireguard-go
make libwg
mv libwg.* ../
