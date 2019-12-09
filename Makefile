STACK_NAME ?= CDKToolkit

.DEFAULT_GOAL := build

.PHONY: install build test deploy synth diff clean

install:
	npm install

build:
	npm run build
	docker run --rm -v ${PWD}/lambda/hello:/code -v ${HOME}/.cargo/registry:/root/.cargo/registry -v ${HOME}/.cargo/git:/root/.cargo/git softprops/lambda-rust

test:
	npm run test

deploy: build
	cdk deploy --toolkit-stack-name $(STACK_NAME)

synth:
	cdk synthesize --toolkit-stack-name ${STACK_NAME}

diff: build
	cdk diff --toolkit-stack-name ${STACK_NAME}

clean:
	cdk destroy --toolkit-stack-name ${STACK_NAME}