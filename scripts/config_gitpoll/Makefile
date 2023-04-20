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
