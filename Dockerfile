FROM alpine:3.16.2 as builder

ARG AIA_URL_SERVER_PORT

RUN apk update 
RUN apk add openssl make
RUN apk add m4

WORKDIR /identity

COPY pki/Makefile .
COPY pki/req req
COPY pki/extensions.conf .
COPY pki/tests tests

RUN AIA_URL_SERVER_PORT=$AIA_URL_SERVER_PORT make clean install tests

RUN tar cvzf identity.tar.gz pki
RUN mv identity.tar.gz pki/ciph.xxx

WORKDIR /tls

COPY tls/Makefile .
COPY tls/conf conf
COPY tls/extensions.conf .

RUN --mount=type=secret,id=tls-password,required --mount=type=secret,id=tls-ca-password,required  PASSWORD_PATH=/run/secrets/ make clean install

WORKDIR /server

COPY server/nginx.conf nginx.conf
COPY server/Makefile Makefile

RUN make clean all

FROM nginx:1.21.6-alpine

RUN rm -fr /etc/nginx/conf.d
COPY --from=builder /server/nginx /etc/nginx/conf.d
COPY --from=builder /identity/pki /var/pki
COPY --from=builder /tls/tls /etc/nginx/tls
COPY server/mime.types /etc/nginx/mime.types
COPY server/root.sh /root.sh

RUN chmod 755 /root.sh
