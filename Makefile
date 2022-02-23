include tools/.env

new:
	docker-compose -f tools/docker-compose-tools.yaml run --rm -v "$$(pwd)":/app -w /app wasm cargo new app-web --lib

build:
	docker-compose -f tools/docker-compose-tools.yaml run --rm -v "$$(pwd)/app-web":/app -w /app wasm wasm-pack build

deps:
	docker-compose -f tools/docker-compose-tools.yaml run --rm -v "$$(pwd)/app-web":/app -w /app node yarn install

yarn-%:
	docker-compose -f tools/docker-compose-tools.yaml run --rm -v "$$(pwd)/app-web":/app -w /app node yarn run $*

run:
	docker-compose -f tools/docker-compose.yaml -p $(PROJECT_NAME) up -d

down:
	docker-compose -f tools/docker-compose.yaml -p $(PROJECT_NAME) down
