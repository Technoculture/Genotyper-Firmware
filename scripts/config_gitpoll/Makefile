PROJ=geno
NAME=gitpoll

start:
	./$(NAME)_daemon.py

pid:
	cat /tmp/$(NAME)_daemon.pid

logs:
	tail -f /tmp/$(PROJ)_$(NAME)_daemon.log

kill:
	kill -9 `cat /tmp/$(NAME)_daemon.pid`

test:
	python $(NAME)/test_$(NAME).py

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
	@echo "  pid\t\tShow the daemon's pid"
	@echo "  help\t\tShow this help"

