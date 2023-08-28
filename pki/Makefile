OPENSSL := openssl

define parse_config
$(foreach pair,$(shell grep = $(1) | sed -e 's/#.*//g' -e 's/^[ \t]*//' -e 's/[ \t]*=/:=/' -e 's/=[ \t]*/=/' -e 's/ /___space___/g'),$(eval $(subst ___space___, ,$(pair))))
endef

REQ_DIR := req
BUILD_DIR := target
CERT_DIR := $(BUILD_DIR)
KEY_DIR := $(BUILD_DIR)
TEST_DIR := tests
INSTALL_DIR_CERT := pki
INSTALL_DIR_KEY := key

EXTENSION_FILE := $(realpath extensions.conf)

export EXTENSION_FILE

.PRECIOUS: $(KEY_DIR)/%.key $(CERT_DIR)/%.cer $(CERT_DIR)/%.pem $(CERT_DIR)/%.chain

all: certs

$(KEY_DIR)/%.key:
	mkdir -p $(dir $@)
	$(OPENSSL) genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -outform der -out $@

$(CERT_DIR)/%.req: $(REQ_DIR)/%.conf $(EXTENSION_FILE) $(KEY_DIR)/%.key
	mkdir -p $(dir $@)
	$(OPENSSL) req -new -config $< -keyform der -key $(word 3,$^) -out $@

$(CERT_DIR)/%.cer: $(CERT_DIR)/%.req $(REQ_DIR)/%.ca
	$(call parse_config,$(word 2,$^))
	mkdir -p $(dir $@)
	if [ $(authority) ]; then \
		$(MAKE) $(CERT_DIR)/$(authority).pem ; \
		$(OPENSSL) x509 -req -days 1 -CAcreateserial -outform der -extfile $(word 2,$^) -CAform der -CAkeyform der -CA $(CERT_DIR)/$(authority).cer -CAkey $(KEY_DIR)/$(authority).key -in $< -out $@ ; \
	else \
		$(OPENSSL) x509 -req -days 1 -CAcreateserial -outform der -extfile $(word 2,$^) -keyform der -signkey $(basename $(@:$(CERT_DIR)/%=$(KEY_DIR)/%)).key -in $< -out $@ ; \
	fi;

$(CERT_DIR)/%.pem: $(CERT_DIR)/%.cer
	$(OPENSSL) x509 -inform der -in $< -outform pem -out $@

$(CERT_DIR)/%.pubkey: $(CERT_DIR)/%.p7c
	$(OPENSSL) x509 -inform der -in $(<:%.p7c=%.cer) -pubkey -noout > $@

$(CERT_DIR)/%.crossreq: $(KEY_DIR)/%.key
	$(OPENSSL) req -new -config $(REQ_DIR)/$(SRC).conf -keyform der -key $< -outform pem -out $@
	rm $<

$(CERT_DIR)/%.cross: $(CERT_DIR)/%.crossreq $(CERT_DIR)/$(SRC).pubkey $(EXTENSION_FILE)
	echo CA: $(CA) > $@
	echo SRC: $(SRC) >> $@
	$(OPENSSL) x509 -req -days 1 -CAcreateserial -outform der -extfile $(CROSS) -CAform der -CAkeyform der -CA $(CERT_DIR)/$(CA).cer -CAkey $(KEY_DIR)/$(CA).key -in $< -force_pubkey $(word 2,$^) -out $(@:%.cross=%.cer)
	$(OPENSSL) x509 -inform der -in $(@:%.cross=%.cer) -outform pem -out $(@:%.cross=%.pem)

$(CERT_DIR)/%.chain: $(CERT_DIR)/%.pem
	$(call parse_config,$(@:$(CERT_DIR)/%.chain=$(REQ_DIR)/%.ca))
	$(MAKE) chain CA=$(authority) REQ=$(@:$(CERT_DIR)/%.chain=$(REQ_DIR)/%.conf) CHAIN=$@

chain:
	touch $(CHAIN)
ifdef CA
	$(call parse_config,$(REQ_DIR)/$(CA).ca)
	$(MAKE) chain CA=$(authority) REQ=$(REQ_DIR)/$(CA).conf
	cat $(REQ:$(REQ_DIR)/%.conf=$(CERT_DIR)/%.pem) $(CHAIN) > $(CHAIN).t
	mv $(CHAIN).t $(CHAIN)
endif

$(CERT_DIR)/%.p7c: $(CERT_DIR)/%.chain
	$(if $(shell if [ -s "$<" ]; then echo true; fi), \
		$(OPENSSL) crl2pkcs7 -nocrl -certfile $(@:%.p7c=%.chain) -outform der -out $@, \
		touch $@ \
	)

cross:
	$(call parse_config,$(CROSS))
	$(MAKE) SRC=$(source) CA=$(authority) $(CROSS:$(REQ_DIR)/%.cross=$(CERT_DIR)/%.cross)

certs:
	$(foreach req, $(wildcard $(REQ_DIR)/*/*.conf), \
		$(MAKE) $(req:$(REQ_DIR)/%.conf=$(CERT_DIR)/%.p7c) || exit  ; \
	)
	$(foreach cross, $(wildcard $(REQ_DIR)/*/*.cross), \
		$(MAKE) cross CROSS=$(cross) || exit  ; \
	)

test:
	$(foreach conf, $(wildcard $(TEST_DIR)/*.conf), \
		$(call parse_config,$(conf)) \
		$(OPENSSL) verify -show_chain -verbose -x509_strict -CAfile $(CERT_DIR)/$(CA).pem $(foreach untrusted, $(IC), -untrusted $(CERT_DIR)/$(untrusted).pem) $(CERT_DIR)/$(PKC).pem || exit ; \
	)

install: certs
	$(foreach src, $(wildcard \
				$(BUILD_DIR)/*/*.cer \
				$(BUILD_DIR)/*/*.pem \
				$(BUILD_DIR)/*/*.p7c \
				$(BUILD_DIR)/*/*.chain \
			),\
		$(MAKE) install_file SRC=$(src) DEST=$(patsubst %.chain,%-fullchain.pem,$(src:$(BUILD_DIR)/%=$(INSTALL_DIR_CERT)/%)) ; \
	)
	$(foreach src, $(wildcard $(BUILD_DIR)/*/*.key),\
		$(MAKE) install_file SRC=$(src) DEST=$(src:$(BUILD_DIR)/%=$(INSTALL_DIR_KEY)/%) ; \
	)
	
install_file:
	$(if $(shell if [ -s "$(SRC)" ]; then echo true; fi), \
		mkdir -p $(dir $(DEST)) ; \
		cp $(SRC) $(DEST) ; \
	)


clean:
	rm -fr $(BUILD_DIR)
	rm -fr $(INSTALL_DIR_CERT)
	rm -fr $(INSTALL_DIR_KEY)
