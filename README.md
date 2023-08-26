# X509 Test Framework


Generate PKI and TLS certificates and key pairs

```` shell
export SERVER_KEY_PASSWORD=$PWD/password.txt
export CA_ROOT_PASSWORD=$PWD/ca-password.txt

rm -fr $PWD/docker/material

docker run --rm -v $PWD/docker/material:/dest $(docker build --secret id=password,src=$PWD/docker/password.txt --secret id=password-ca,src=$PWD/docker/password-ca.txt -f $PWD/docker/material.Dockerfile -q $PWD/docker)
````

Build and launch the server

```` shell
export FRAMEWORK_MATERIAL=$PWD/target/tmp/material

rm -fr $PWD/docker/server/material
cp -r $FRAMEWORK_MATERIAL $PWD/docker/server/material


````
