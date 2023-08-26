#!/bin/sh

mkdir -p /dest/identity
cp -r /identity/pki /dest/identity/x509
cp -r /identity/key /dest/identity

cp -r /tls/tls /dest

exit 0
