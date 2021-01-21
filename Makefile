install:
	cd www && npm install

build:
	date
	wasm-pack build

start:
	cd www && npm run start

clean:
	rm -rf pkg