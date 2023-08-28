#!/bin/sh

cat
if [ ! -f /etc/nginx/tls/password.txt ]; then printenv ENCRYPTED_TLS_KEY_PASSWORD > etc/nginx/tls/password.txt; fi

exit 0
