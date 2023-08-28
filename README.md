# PKI Test Framework

PKI Test Framework is a dockerized test PKI generator, with a built-in HTTP/S certificate server.

## Synopsis

PKI Test Framework has three components:

1. Multi-organization PKI identity tree generator
2. TLS server certificate generator
3. Certificate web server

Both the identity tree and TLS server certificates are generated from scratch every time the container builds.

## Usage

Clone from Github:
``` shell
git clone xxx
cd xxx
```

Build and run:
``` shell
docker compose build 
docker compose up   
```

Bootstrap root TLS:
``` shell
docker compose exec identity /root.sh -t > root.pem
```

Download a certificate
``` shell
curl --cacert root.pem https://identity.vandelaybank.com:4443/certificates/kim@id.vandelaybank.com.pem
```

Install PKI identities locally:
``` shell
mkdir -p target
cd target
 curl --cacert ../root.pem https://identity.ciph.xxx:4443/certificates/identity.tar.gz |
tar xzv - 
```
