OPENSSL := openssl

define parse_config
$(foreach pair,$(shell grep = $(1) | sed -e 's/#.*//g' -e 's/^[ \t]*//' -e 's/[ \t]*=/:=/' -e 's/=[ \t]*/=/' -e 's/ /___space___/g'),$(eval $(subst ___space___, ,$(pair))))
endef

REQ_DIR := conf
BUILD_DIR := target
CERT_DIR := $(BUILD_DIR)
KEY_DIR := $(BUILD_DIR)
TEST_DIR := tests
INSTALL_DIR_CERT := tls
INSTALL_DIR_KEY := tls

EXTENSION_FILE := $(realpath extensions.conf)

export EXTENSION_FILE

.PRECIOUS: $(KEY_DIR)/%.key $(CERT_DIR)/%.cer $(CERT_DIR)/%.pem $(CERT_DIR)/%.chain

all: certs

$(KEY_DIR)/%.key:
	$(call parse_config,$(@:$(KEY_DIR)/%.key=$(REQ_DIR)/%.ca))
	mkdir -p $(dir $@)
	$(OPENSSL) genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -outform pem -pass file:$(PASSWORD_PATH)$(password) -aes256 -out $@

$(CERT_DIR)/%.req: $(REQ_DIR)/%.conf $(EXTENSION_FILE) $(KEY_DIR)/%.key
	$(call parse_config,$(<:%.conf=%.ca))
	mkdir -p $(dir $@)
	$(OPENSSL) req -new -config $< -keyform pem -key $(word 3,$^) -passin file:$(PASSWORD_PATH)$(password) -out $@

$(CERT_DIR)/%.cer: $(CERT_DIR)/%.req $(REQ_DIR)/%.ca
	$(call parse_config,$(word 2,$^))
	$(eval ca_authority := $(authority))
	$(call parse_config,$(REQ_DIR)/$(authority).ca)
	mkdir -p $(dir $@)
	if [ $(ca_authority) ]; then \
		$(MAKE) $(CERT_DIR)/$(ca_authority).pem ; \
		$(OPENSSL) x509 -req -days 1 -CAcreateserial -outform der -extfile $(word 2,$^) -CAform der -CAkeyform pem -CA $(CERT_DIR)/$(ca_authority).cer -CAkey $(KEY_DIR)/$(ca_authority).key -passin file:$(PASSWORD_PATH)$(password) -in $< -out $@ ; \
	else \
		$(OPENSSL) x509 -req -days 1 -CAcreateserial -outform der -extfile $(word 2,$^) -keyform pem -signkey $(basename $(@:$(CERT_DIR)/%=$(KEY_DIR)/%)).key -passin file:$(PASSWORD_PATH)$(password) -in $< -out $@ ; \
	fi;

$(CERT_DIR)/%.pem: $(CERT_DIR)/%.cer
	$(OPENSSL) x509 -inform der -in $< -outform pem -out $@

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

certs:
	$(foreach req, $(wildcard $(REQ_DIR)/*/*.conf), \
		$(MAKE) $(req:$(REQ_DIR)/%.conf=$(CERT_DIR)/%.p7c) || exit  ; \
	)

install: certs
	$(foreach src, $(wildcard \
				$(BUILD_DIR)/*/*.cer \
				$(BUILD_DIR)/*/*.pem \
				$(BUILD_DIR)/*/*.p7c \
			),\
		$(MAKE) install_file SRC=$(src) DEST=$(src:$(BUILD_DIR)/%=$(INSTALL_DIR_CERT)/%) ; \
	)
	$(foreach src, $(wildcard $(BUILD_DIR)/*/*.chain),\
		$(MAKE) install_file SRC=$(src) DEST=$(patsubst %.chain,%.pem,$(src:$(BUILD_DIR)/%=$(INSTALL_DIR_CERT)/%)) ; \
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
