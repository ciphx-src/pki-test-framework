FROM alpine:3.16.2 as builder

RUN apk update 
RUN apk add openssl make
RUN apk add m4

WORKDIR /identity

COPY identity/Makefile .
COPY identity/req req
COPY identity/extensions.conf .
COPY identity/tests tests

RUN make clean install tests

RUN tar cvzf identity.tar.gz pki
RUN mv identity.tar.gz pki/ciph.xxx

WORKDIR /tls

COPY tls/Makefile .
COPY tls/conf conf
COPY tls/extensions.conf .

RUN --mount=type=secret,id=password,dst=password.txt --mount=type=secret,id=password-ca,dst=password-ca.txt make clean install

WORKDIR /server

COPY server/nginx.conf nginx.conf
COPY server/Makefile Makefile

RUN make clean all

FROM nginx:1.21.6-alpine

RUN rm -fr /etc/nginx/conf.d
COPY --from=builder /server/nginx /etc/nginx/conf.d
COPY --from=builder /identity/pki /var/pki
COPY --from=builder /tls/tls /etc/nginx/tls
COPY --from=builder /tls/tls/ciph.xxx/tls.ciph.xxx.pem /var/pki/ciph.xxx/root.pem

COPY server/mime.types /etc/nginx/mime.types
COPY server/40-set-encrypted-tls-key-password.sh /docker-entrypoint.d/40-set-encrypted-tls-key-password.sh

RUN chmod 775 /docker-entrypoint.d/40-set-encrypted-tls-key-password.sh
