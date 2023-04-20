COMPANY=tcr
PROJ=geno
NAME=gitpoll

CONTAINER_NAME:=$(PROJ)_$(NAME)_foreground
IMAGE_NAME:=$(COMPANY)/$(PROJ)_$(NAME)_foreground

help:
	@echo "gitpoll: simple git polling daemon"
	@echo ""
	@echo "usage:"
	@echo "  make <target>"
	@echo ""
	@echo "targets:"
	@echo "  start\t\tStart the daemon"
	@echo "  kill\t\tKill the daemon"
	@echo "  logs\t\tShow the daemon's logs"
	@echo "  test\t\tRun the tests"
	@echo "  --\t\t--"
	@echo "  dbuild\tBuild the docker image"
	@echo "  drun\t\tRun the docker image"
	@echo "  dlogs\t\tShow the docker container's logs"
	@echo "  --\t\t--"
	@echo "  pid\t\tShow the daemon's pid"
	@echo "  help\t\tShow this help"

start:
	./$(NAME)_daemon.py

pid:
	cat /tmp/$(NAME)_daemon.pid

logs:
	tail -f /tmp/$(NAME)_daemon.log

logs:
	tail -f /tmp/$(NAME)_daemon.log

dlogs:
	tail -f /tmp/$(NAME)_foreground.log

kill:
	kill -9 `cat /tmp/$(NAME)_daemon.pid`

test:
	python $(NAME)/test_$(NAME).py

dbuild:
	docker build -t $(IMAGE_NAME) .

drun:
	docker run -it -v /tmp:/tmp $(IMAGE_NAME)

