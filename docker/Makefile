DOCKER := docker

ifndef TARGET
	TARGET := ../target/tmp/pki
endif

all:
	mkdir -p $(TARGET)/material
	cp -r pki $(TARGET)
	cp pki.sh $(TARGET)
	$(DOCKER) build -q --secret id=password,src=../password.txt --secret id=password-ca,src=../password-ca.txt -f pki.Dockerfile $(TARGET)
#	$(DOCKER) run --rm -v $(abspath $(TARGET)/material):/dest $()

#clean:
#	rm -fr $(dir $@)/docker

