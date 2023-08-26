FROM alpine:3.16.2 as builder

ARG AIA_SUBDOMAN

RUN apk update 
RUN apk add openssl make

WORKDIR /identity

COPY pki/identity/Makefile .
COPY pki/identity/req req
COPY pki/identity/extensions.conf .
COPY pki/identity/tests tests

RUN make clean install tests AIA_SUBDOMAN=$AIA_SUBDOMAN

WORKDIR /tls

COPY pki/tls/Makefile .
COPY pki/tls/conf conf
COPY pki/tls/extensions.conf .

RUN --mount=type=secret,id=password,dst=password.txt --mount=type=secret,id=password-ca,dst=password-ca.txt make clean install

