OPENSSL := openssl
M4 := m4

BUILD_DIR = build
TLS_DIR := /tls/tls
NGINX_DIR := nginx

servers := $(wildcard $(TLS_DIR)/*/identity.*.pem)

all: $(servers:$(TLS_DIR)/%.pem=$(BUILD_DIR)/%.conf)

$(BUILD_DIR)/%.conf: nginx.conf
	mkdir -p $(dir $@)
	$(M4) --define org_name=$(word 2,$(subst /, ,$@)) --define domain_name=$(notdir $(basename $@)) $< > $@
	mkdir -p $(NGINX_DIR)
	cp $@ $(NGINX_DIR)

clean:
	rm -fr $(BUILD_DIR)
	rm -fr $(NGINX_DIR)

