DOCKER := docker

ifndef TARGET
	TARGET := ../target/tmp/pki
endif

all:
	mkdir -p $(TARGET)
	cp -r pki $(TARGET)/docker
	cp pki.sh $(TARGET)/docker
	$(DOCKER) build -q --secret id=password,src=../password.txt --secret id=password-ca,src=../password-ca.txt -f pki.Dockerfile $(TARGET)/docker > $(TARGET)/image.hash
	$(DOCKER) run --rm -v $(abspath $(TARGET))/material:/dest $(shell cat $(TARGET)/image.hash)

#clean:
#	rm -fr $(dir $@)/docker

