# X509 Test Framework


* Generate PKI identities
* Generate server TLS certificates and keys
* Build http server
* Launch the server

```` shell
docker run --rm --env-file password-local.env -p 127.0.0.1:8080:80 -p 127.0.0.1:4443:443 $(docker build -q --secret id=password,src=password.txt --secret id=password-ca,src=password.txt docker)

````

Bootstrap TLS root

```` shell
curl 'http://identity.ciph.xxx:8080/root.pem' > root.pem
````

Install PKI identities

```` shell
 curl --cacert root.pem https://identity.ciph.xxx:4443/identity.tar.gz > identity.tar.gz

tar fxzv identity.tar.gz
````

