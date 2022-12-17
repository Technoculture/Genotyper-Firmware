help:
	@echo "usage"
	@echo "  make [target]"
	@echo ""
	@echo "available targets"
	@echo "  run    Upload onto the Microcontroller"
	@echo "  test   Run unit tests"
	@echo "  all    Run all targets"
	@echo ""
	@echo "This is a convinience Makefile, is likely to only work on Linux, Mac Os and WSL.\nConsult satyam@technoculture.io or write your own makefile if facing issues."

all: db_ test run

db_:
	pio run -t compiledb

run: db_
	pio run -e firmware

test: db_
	pio test -e native

docker-start:
	sudo service docker start

docker-build:
	sudo docker build -t tcr/orchestrator .

docker-run: docker-build
	sudo docker run tcr/orchestrator

.PHONY: docker-start docker-build docker-run run test
