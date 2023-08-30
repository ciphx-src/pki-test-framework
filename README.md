# PKI Test Framework

PKI Test Framework is a dockerized test PKI generator, with a built-in HTTP/S certificate server.

![CI Status](https://github.com/merlincinematic/pki-test-framework/actions/workflows/ci.yaml/badge.svg)

## Synopsis

PKI Test Framework is designed to produce X509 material useful for CI/CD integration testing.

On build, PKI Test Framework will generate a multi-organization PKI identity tree, then launch a web server to serve the certificates. The certificates are available over HTTPS, so TLS certificates are generated as well.

## Usage

Clone from GitHub:
``` shell
git clone https://github.com/merlincinematic/pki-test-framework.git
cd pki-test-framework
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

Optionally, install PKI identities locally for file access:
``` shell
mkdir -p target
cd target
 curl --cacert ../root.pem https://identity.ciph.xxx:4443/certificates/identity.tar.gz |
tar xzv - 
```

### Configuration

#### Build Arguments

* `AIA_URL_SERVER_PORT` - when generating the PKI identity tree, use this port for [AIA](https://datatracker.ietf.org/doc/html/rfc5280#section-5.2.7) URLs. It should match the port the certificate server is running on.
* `EXPIRES` - sets the expiration validity date on all certificates. Argument is required, and has no default. The [CA/Browser Forum](https://cabforum.org/wp-content/uploads/CA-Browser-Forum-BR-1.8.1.pdf) recommends that TLS certificates expire after 398 days. As a result of the recommendation, most browsers will reject certificates with a longer lifespan.

#### Secrets

The `tls-password.txt` and `tls-ca-password.txt` files hold the passwords for encrypting the TLS keys used for the server certificates.

## Identity

PKI Test Framework builds three fictional organizations:

1. biffgroup.com
2. bluthdigital.com
3. vandelaybank.com

Each organization has its own root authority. All certificates contain an [AIA](https://datatracker.ietf.org/doc/html/rfc5280#section-5.2.7) extension, which links to where its issuer certificate can be accessed from certificate server.

bluthdigital.com and vandelaybank.com are cross-signed with the biffgroup.com bridge authority.

Path Validation Example

![biffgroup.com](./doc/cross-paths.png#2)

### Organization PKI Details

#### biffgroup.com

Root name: CN=biffgroup.com

Certificate server base URL: https://identity.biffgroup.com:4443/certificates/

biffgroup.com is a [bridge](https://datatracker.ietf.org/doc/html/rfc4158#section-1.5.4) authority. It issues [cross-signed](https://datatracker.ietf.org/doc/html/rfc4158#section-1.5.3) two certificates:

1. bridge.biffgroup.com <-> id.bluthdigital.com 
2. bridge.biffgroup.com <-> id.vandelaybank.com

##### Certificate Issuer Paths

![biffgroup.com](./doc/biffgroup.com.png#2)

#### bluthdigital.com

Root name: CN=bluthdigital.com 

Certificate server base URL: https://identity.bluthdigital.com:4443/certificates/

bluthdigital.com is a member of the biffgroup.com bridge. It issues a crossed-signed certificate for bridge.biffgroup.com.

##### Certificate Issuer Paths

![biffgroup.com](./doc/bluthdigital.com.png#2)

#### vandelaybank.com

Root name: CN=vandelaybank.com

Certificate server base URL: https://identity.vandelaybank.com:4443/certificates/

vandelaybank.com is a member of the biffgroup.com bridge. It issues a crossed-signed certificate for bridge.biffgroup.com.

##### Certificate Issuer Paths

![biffgroup.com](./doc/vandelaybank.com.png#2)

### Certificate Encoding Formats

Every certificate is encoded multiple times in the formats listed below. The format be identified by the filename extension:

* `.cer` - single DER-encoded certificate
* `.pem` - single PEM-encoded certificate
* `.p7c` - DER-encoded PKCS7 certificate bundle of the complete certificate chain, excluding root
* `-fullchain.pem` - PEM encoded stack of the complete certificate chain, excluding root

## Certificate Server

### DNS and URL patterns

The certificate server has real DNS entries, with an `A` record that resolves to `127.0.0.1`. Each organization in the identity tree has its own domain. The URL pattern is:

``` text
https://identity.{organization-name}/certificates/{certificate-name}.{format-extension}
```

Example:

For the organization vandelaybank.com, the certificate named kim@id.vandelaybank.com can be downloaded in `PEM` format from:

```text
https://identity.vandelaybank.com:4443/certificates/kim@id.vandelaybank.com.pem
```

### TLS

On build, TLS certificates are generated for each organization, with a single root authority used to sign all certificates. The root can be bootstrapped as follows:

1. Extract the root from a running container: `docker compose exec identity /root.sh -t > root.pem`
2. Install the root into your trusted certificate store. Curl example: `curl --cacert root.pem {url}`

