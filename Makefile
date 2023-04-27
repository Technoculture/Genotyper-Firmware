COMPANY=tcr
PROJ=geno
NAME=dev

CONTAINER=$(COMPANY)/$(PROJ)_$(NAME)
IMAGE_NAME=$(CONTAINER):latest

devbuild:
	docker build -f Dockerfile.dev -t $(IMAGE_NAME) .

dshell: devbuild
	docker run -it --rm -v $(PWD):/app -w /app $(IMAGE_NAME) /bin/bash

devrun: devbuild
	docker run -it --rm -v $(PWD):/app -w /app $(IMAGE_NAME) /bin/bash -c "cargo run"

