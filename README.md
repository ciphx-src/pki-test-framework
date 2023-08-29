# PKI Test Framework

PKI Test Framework is a dockerized test PKI generator, with a built-in HTTP/S certificate server.

## Synopsis

On build, PKI Test Framework will generate a multi-organization PKI identity tree, then launch a web server to serve the certificates. The certificate are available over HTTPS, so TLS certificates are generated as well.

## Usage

Clone from Github:
``` shell
git clone xxx
cd xxx
```

Build and run:
``` shell
docker compose build --build-arg "EXPIRES=yyyymmdd"
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

### Configuration

#### Build Arguments

* `AIA_URL_SERVER_PORT` - when generating the PKI identity tree, use this port for AIA urls. It should match the port the certificate server is running on.
* `EXPIRES` - sets the expiration validity date on all certificates. Argument is required, and has no default. The [CA/Browser Forum](https://cabforum.org/wp-content/uploads/CA-Browser-Forum-BR-1.8.1.pdf) recommends TLS certificates must expire in 398 days. As a result of the recomendation, most browsers will reject certificates with a longer lifespan.

#### Secrets

The `tls-password.txt` and `tls-ca-password.txt` files hold the passwords for encrypting the TLS keys used for the server certificates.

## Identity

PKI Test Framework build the following fictional organizations:

* ajaxmetrics.com
* biffgroup.com
* bluthdigital.com
* cogswelldata.com
* vandelaybank.com



#### Certificate Encoding Formats

Each identity certificate is encoded in the following formats, which can be identified by the filename extension:

* `.cer` - single DER-encoded certificate
* `.pem` - single PEM-encoded certificate
* `.p7c` - DER-encoded PKCS7 certificate bundle of the complete certificate chain, excluding root
* `-fullchain.pem` - PEM encoded stack of the complete certificate chain, excluding root

## Certificate Server

### DNS and URL patterns

The certificate server has real DNS entries, with an A record that resolves to `127.0.0.1`. Each organization in the identity tree has its own domain. The URL pattern is:

``` text
https://identity.{organization-name}/certificates/{certificate-name}.{format-extension}
```

Example:

For the organization "vandelaybank.com", the certificate named "kim@id.vandelaybank.com" can be downloaded in `PEM` format from:

```text
https://identity.vandelaybank.com:4443/certificates/kim@id.vandelaybank.com.pem
```

### TLS

On build, TLS certificates are generated for each organization, with a single root authority used to sign all certificates. The root can be bootstrapped as follows:

1. Extract the root from a running container: `docker compose exec identity /root.sh -t > root.pem`
2. Install the root into your trusted certificate store. Curl example: `curl --cacert root.pem {url}`

