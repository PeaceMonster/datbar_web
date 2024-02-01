default: build server

build:
	cd frontend; npm run build

dev: 
	cd frontend; npm run dev

server:
	RUST_LOG=debug cargo run