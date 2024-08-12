IMAGE  := ghcr.io/mhutter/glbt
DOCKER := docker

.PHONY: container
container:
	trunk clean
	trunk build --release
	$(DOCKER) build -t "$(IMAGE)" .
